use std::time::{Duration, Instant};

use crate::research::{ExperimentResult, ProofStatus};
use crate::spreadsheet::{Cell, DataTable};

#[derive(Debug, Clone)]
pub struct ExperimentSpec {
    pub id: String,
    pub algorithm: String,
    pub input_range: String,
    pub samples: u64,
}

impl ExperimentSpec {
    pub fn new(id: &str, algorithm: &str, input_range: &str, samples: u64) -> Self {
        Self { id: id.into(), algorithm: algorithm.into(), input_range: input_range.into(), samples }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExecutionConfig {
    pub workers: usize,
}

impl Default for ExecutionConfig {
    fn default() -> Self { Self { workers: 1 } }
}

#[derive(Debug, Clone)]
pub struct ExperimentRun {
    pub result: ExperimentResult,
    pub duration: Duration,
}

pub struct ExperimentRunner {
    config: ExecutionConfig,
}

impl ExperimentRunner {
    pub fn new(config: ExecutionConfig) -> Self {
        Self { config: ExecutionConfig { workers: config.workers.max(1) } }
    }

    pub fn config(&self) -> ExecutionConfig {
        self.config
    }

    pub fn record<F>(&self, spec: &ExperimentSpec, operation: F) -> ExperimentRun
    where
        F: FnOnce(&ExperimentSpec) -> (u64, u64, Option<String>),
    {
        let started = Instant::now();
        let (tested, passed, counterexample) = operation(spec);
        let mut result = ExperimentResult::new(&spec.id, &spec.algorithm, &spec.input_range);
        result.tested = tested;
        result.passed = passed;
        result.counterexample = counterexample;
        result.proof_status = if result.counterexample.is_some() {
            ProofStatus::Disproved
        } else {
            ProofStatus::ComputationalEvidence
        };
        ExperimentRun { result, duration: started.elapsed() }
    }

    pub fn result_table(run: &ExperimentRun) -> DataTable {
        let mut table = DataTable::new(
            "experiment_results",
            vec!["experiment_id", "algorithm", "input_range", "tested", "passed", "counterexample", "proof_status", "duration_ms"],
        );
        let counterexample = run.result.counterexample.clone().unwrap_or_default();
        let status = format!("{:?}", run.result.proof_status);
        table.add_row(vec![
            Cell::Text(run.result.experiment_id.clone()),
            Cell::Text(run.result.algorithm.clone()),
            Cell::Text(run.result.input_range.clone()),
            Cell::Number(run.result.tested as f64),
            Cell::Number(run.result.passed as f64),
            Cell::Text(counterexample),
            Cell::Text(status),
            Cell::Number(run.duration.as_secs_f64() * 1000.0),
        ]).expect("runner generated a valid row");
        table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runner_records_evidence() {
        let runner = ExperimentRunner::new(ExecutionConfig { workers: 4 });
        let spec = ExperimentSpec::new("e1", "goldbach", "2..100", 50);
        let run = runner.record(&spec, |_| (50, 50, None));
        assert_eq!(run.result.tested, 50);
        assert_eq!(run.result.passed, 50);
        assert_eq!(run.result.proof_status, ProofStatus::ComputationalEvidence);
        assert_eq!(runner.config().workers, 4);
    }

    #[test]
    fn counterexample_marks_disproof() {
        let runner = ExperimentRunner::default();
        let spec = ExperimentSpec::new("e2", "example", "1..10", 10);
        let run = runner.record(&spec, |_| (10, 9, Some("10".into())));
        assert_eq!(run.result.proof_status, ProofStatus::Disproved);
    }
}

impl Default for ExperimentRunner {
    fn default() -> Self { Self::new(ExecutionConfig::default()) }
}
