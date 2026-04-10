use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GovernedFlowStep {
    pub step: &'static str,
    pub detail: String,
    pub passed: bool,
}

impl GovernedFlowStep {
    pub fn pass(step: &'static str, detail: impl Into<String>) -> Self {
        Self {
            step,
            detail: detail.into(),
            passed: true,
        }
    }

    pub fn fail(step: &'static str, detail: impl Into<String>) -> Self {
        Self {
            step,
            detail: detail.into(),
            passed: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GovernedFlowReport {
    pub scenario_id: &'static str,
    pub steps: Vec<GovernedFlowStep>,
    pub triggering_event_ids: Vec<String>,
    pub affected_artifact_ids: Vec<String>,
    pub unaffected_artifact_ids: Vec<String>,
    pub affected_packet_ids: Vec<String>,
    pub unaffected_packet_ids: Vec<String>,
    pub initial_artifact_freshness: String,
    pub final_artifact_freshness: String,
    pub initial_packet_admissibility: String,
    pub final_packet_admissibility: String,
    pub deduped_events: usize,
    pub coalesced_batches: usize,
    pub remediation_required: bool,
    pub remediation_count: usize,
    pub schema_paths: Vec<PathBuf>,
}

impl GovernedFlowReport {
    pub fn passed(&self) -> bool {
        self.steps.iter().all(|step| step.passed)
    }
}
