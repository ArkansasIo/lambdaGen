#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofStatus { ComputationalEvidence, ProvenTheorem, Disproved }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperimentResult {
    pub experiment_id: String,
    pub algorithm: String,
    pub input_range: String,
    pub tested: u64,
    pub passed: u64,
    pub counterexample: Option<String>,
    pub proof_status: ProofStatus,
}

impl ExperimentResult {
    pub fn new(id: &str, algorithm: &str, range: &str) -> Self {
        Self { experiment_id: id.into(), algorithm: algorithm.into(), input_range: range.into(), tested: 0, passed: 0, counterexample: None, proof_status: ProofStatus::ComputationalEvidence }
    }
}
