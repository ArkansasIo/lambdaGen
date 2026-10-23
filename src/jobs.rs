use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus { Queued, Running, Succeeded, Failed }

#[derive(Debug, Clone)]
pub struct Job { pub id: String, pub payload: String, pub status: JobStatus }

#[derive(Debug, Default)]
pub struct JobQueue { queue: VecDeque<Job> }

impl JobQueue {
    pub fn new() -> Self { Self::default() }
    pub fn enqueue(&mut self, id: impl Into<String>, payload: impl Into<String>) { self.queue.push_back(Job { id:id.into(), payload:payload.into(), status:JobStatus::Queued }); }
    pub fn next(&mut self) -> Option<Job> { self.queue.pop_front().map(|mut j| { j.status=JobStatus::Running; j }) }
    pub fn len(&self) -> usize { self.queue.len() }
}

/// Cron-compatible schedule description. Scheduling is intentionally separated from execution.
#[derive(Debug, Clone)]
pub struct CronJob { pub name: String, pub expression: String, pub command: String, pub enabled: bool }
