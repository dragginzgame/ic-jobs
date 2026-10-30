//! Host-only qualification receipts and cohort comparisons; never deployed in probes.

use crate::build::{Cohort, Qualification};
use ic_build::artifacts::WasmBuildRecord;
use ic_host_artifacts::artifact::{hash_reader, read_reader};
use ic_host_fs::durable::{PublicationMode, WriteOptions, write_bytes};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, error::Error, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const SCHEMA: u32 = 2;

// Each field is declared once for construction, strict decoding and comparison.
macro_rules! metrics {
    ($name:ident { $($field:ident),* $(,)? }) => {
        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        pub(crate) struct $name {
            $(pub(crate) $field: u128,)*
        }
        impl $name {
            fn deltas(&self, baseline: &Self, subject: &str, changes: &mut BTreeMap<String, i128>) -> Result<()> {
                $(changes.insert(
                    format!("{subject}/{}", stringify!($field)),
                    i128::try_from(self.$field)?.checked_sub(i128::try_from(baseline.$field)?)
                        .ok_or("delta overflow")?,
                );)*
                Ok(())
            }
        }
    };
}

metrics!(PolicyMetrics {
    start_instructions,
    duplicate_instructions,
    cancel_instructions,
    snapshot_instructions,
    inventory_instructions,
    scheduler_instructions,
    work_instructions,
    memory_sampling_empty,
    memory_sampling_sampled,
    memory_sampling_overhead,
    dispatch_cycles,
    wasm_bytes,
});
metrics!(CalibrationMetrics {
    scheduler_instructions,
    work_instructions,
    dispatch_cycles
});

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Measurements {
    pub(crate) baseline: PolicyMetrics,
    pub(crate) once: PolicyMetrics,
    #[serde(rename = "after-completion")]
    pub(crate) after_completion: PolicyMetrics,
    pub(crate) watchdog: PolicyMetrics,
    #[serde(rename = "watchdog-calibration-minimal")]
    pub(crate) minimal: CalibrationMetrics,
    #[serde(rename = "watchdog-calibration-representative")]
    pub(crate) representative: CalibrationMetrics,
}

impl Measurements {
    const fn policy(&self, cohort: Cohort) -> &PolicyMetrics {
        match cohort {
            Cohort::Baseline => &self.baseline,
            Cohort::Once => &self.once,
            Cohort::AfterCompletion => &self.after_completion,
            Cohort::Watchdog => &self.watchdog,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Context {
    lock_sha256: String,
    server_sha256: String,
    host: String,
    profile: String,
    rustc: String,
    cargo: String,
    target: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Artifact {
    wasm_sha256: String,
    build_fingerprint: String,
    input_digest: String,
    recipe_identity: String,
    feature: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BuildContext {
    schema: u32,
    context: Context,
    artifacts: BTreeMap<String, Artifact>,
}

/// Only a completed measurement has the on-disk report shape.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Report {
    schema: u32,
    context: Context,
    artifacts: BTreeMap<String, Artifact>,
    measurements: Measurements,
}

impl BuildContext {
    pub(crate) fn new(root: &Path) -> Result<Self> {
        let server = std::env::var_os("POCKET_IC_BIN").ok_or("missing admitted POCKET_IC_BIN")?;
        Ok(Self {
            schema: SCHEMA,
            context: Context {
                lock_sha256: digest(&root.join("Cargo.lock"))?,
                server_sha256: digest(Path::new(&server))?,
                host: format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS),
                profile: "timer-probe".into(),
                rustc: String::new(),
                cargo: String::new(),
                target: String::new(),
            },
            artifacts: BTreeMap::new(),
        })
    }

    pub(crate) fn record_build(
        &mut self,
        label: &str,
        feature: &str,
        record: &WasmBuildRecord,
        artifact: &Path,
    ) -> Result<()> {
        let facts = record.facts();
        for (key, previous, value) in [
            (
                "rustc",
                &mut self.context.rustc,
                String::from_utf8(facts.rustc_identity.clone())?,
            ),
            (
                "cargo",
                &mut self.context.cargo,
                String::from_utf8(facts.cargo_identity.clone())?,
            ),
            ("target", &mut self.context.target, facts.target.clone()),
        ] {
            if !previous.is_empty() && *previous != value {
                return Err(format!("probe {key} changed during builds").into());
            }
            *previous = value;
        }
        self.artifacts.insert(
            label.into(),
            Artifact {
                wasm_sha256: digest(artifact)?,
                build_fingerprint: record.fingerprint().to_string(),
                input_digest: record.input_digest().to_string(),
                recipe_identity: record.recipe_identity().to_string(),
                feature: feature.into(),
            },
        );
        Ok(())
    }

    pub(crate) fn verify_context(&self, root: &Path) -> Result<()> {
        let current = Self::new(root)?.context;
        if self.context.lock_sha256 != current.lock_sha256
            || self.context.server_sha256 != current.server_sha256
            || self.context.host != current.host
            || self.context.profile != current.profile
        {
            return Err("qualification context changed during measurement".into());
        }
        Ok(())
    }

    /// The driver calls this only after tests, Build and context checks succeed.
    pub(crate) fn publish_runtime_receipt(
        &self,
        path: &Path,
        qualification: &Qualification,
        subjects: &[String],
    ) -> Result<()> {
        validate_build_context(self.schema, &self.context, &self.artifacts)?;
        if matches!(qualification, Qualification::Cohorts { .. })
            || subjects.is_empty()
            || self.artifacts.len() != 1
            || !self.artifacts.contains_key("runtime")
        {
            return Err("incomplete runtime receipt".into());
        }
        write_bytes(
            path,
            WriteOptions {
                mode: PublicationMode::CreateNew,
                permissions: 0o600,
            },
            &serde_json::to_vec_pretty(&serde_json::json!({
                "selection": qualification.name(),
                "subjects": subjects,
                "verification": "passed",
                "build": self,
            }))?,
        )?;
        println!("runtime receipt: {}", path.display());
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn verify_artifact(&self, cohort: &str, wasm: &[u8]) {
        assert_eq!(
            self.artifacts[cohort].wasm_sha256,
            ic_host_artifacts::artifact::Sha256Digest::compute(wasm).to_string(),
            "measured artifact differs from its build receipt"
        );
    }

    #[cfg(test)]
    pub(crate) fn complete(self, measurements: Measurements) -> Result<Report> {
        let report = Report {
            schema: self.schema,
            context: self.context,
            artifacts: self.artifacts,
            measurements,
        };
        report.validate()?;
        Ok(report)
    }
}

impl Report {
    /// Admit only a complete report from this invocation's child and build context.
    pub(crate) fn read_candidate(path: &Path, expected: &BuildContext) -> Result<Self> {
        let candidate = Self::read(path)?;
        if candidate.schema != expected.schema
            || candidate.context != expected.context
            || candidate.artifacts != expected.artifacts
        {
            return Err("mismatched invocation build context".into());
        }
        candidate.validate()?;
        Ok(candidate)
    }

    fn validate(&self) -> Result<()> {
        validate_build_context(self.schema, &self.context, &self.artifacts)?;
        if self.artifacts.len() != Cohort::ALL.len()
            || !Cohort::ALL.iter().all(|cohort| {
                let name = cohort.name();
                self.artifacts
                    .get(name)
                    .is_some_and(|artifact| artifact.feature == name)
            })
        {
            return Err("incomplete cohort report".into());
        }
        Ok(())
    }

    fn read(path: &Path) -> Result<Self> {
        Ok(serde_json::from_slice(&read_reader(
            fs::File::open(path)?,
            1024 * 1024,
        )?)?)
    }

    pub(crate) fn publish(&self, path: &Path) -> Result<()> {
        self.validate()?;
        write_bytes(
            path,
            WriteOptions {
                mode: PublicationMode::Replace,
                permissions: 0o600,
            },
            &serde_json::to_vec_pretty(self)?,
        )?;
        println!("cohort receipt: {}", path.display());
        Ok(())
    }

    pub(crate) fn compare(&self, baseline: &Path) -> Result<()> {
        let changes = self.deltas(&Self::read(baseline)?)?;
        println!("cohort comparison (review evidence; positive deltas are increases):");
        for (subject, delta) in changes {
            println!("  {subject}: {delta:+}");
        }
        Ok(())
    }

    fn deltas(&self, baseline: &Self) -> Result<BTreeMap<String, i128>> {
        self.validate()?;
        baseline.validate()?;
        if self.context != baseline.context {
            return Err("incomparable cohort compiler/server/host/lock/profile/target; qualify a new baseline explicitly".into());
        }
        if !self.artifacts.iter().all(|(cohort, artifact)| {
            baseline
                .artifacts
                .get(cohort)
                .is_some_and(|previous| previous.recipe_identity == artifact.recipe_identity)
        }) {
            return Err(
                "incomparable cohort build recipes; qualify a new baseline explicitly".into(),
            );
        }
        let current = &self.measurements;
        let previous = &baseline.measurements;
        let mut changes = BTreeMap::new();
        for cohort in Cohort::ALL {
            current
                .policy(cohort)
                .deltas(previous.policy(cohort), cohort.name(), &mut changes)?;
        }
        current.minimal.deltas(
            &previous.minimal,
            "watchdog-calibration-minimal",
            &mut changes,
        )?;
        current.representative.deltas(
            &previous.representative,
            "watchdog-calibration-representative",
            &mut changes,
        )?;
        Ok(changes)
    }
}

fn validate_build_context(
    schema: u32,
    context: &Context,
    artifacts: &BTreeMap<String, Artifact>,
) -> Result<()> {
    if schema != SCHEMA
        || [
            &context.lock_sha256,
            &context.server_sha256,
            &context.host,
            &context.profile,
            &context.rustc,
            &context.cargo,
            &context.target,
        ]
        .iter()
        .any(|value| value.is_empty())
        || artifacts.is_empty()
        || artifacts.values().any(|artifact| {
            [
                &artifact.feature,
                &artifact.wasm_sha256,
                &artifact.build_fingerprint,
                &artifact.input_digest,
                &artifact.recipe_identity,
            ]
            .iter()
            .any(|value| value.is_empty())
        })
    {
        return Err("incomplete qualification build context".into());
    }
    Ok(())
}

pub(crate) fn digest(path: &Path) -> Result<String> {
    Ok(hash_reader(fs::File::open(path)?, 1024 * 1024 * 1024)?
        .sha256
        .to_string())
}

#[cfg(test)]
mod tests;
