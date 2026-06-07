use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::time::{Duration, Instant};

/// Scheduled system task
#[derive(Clone)]
pub struct Task {
    pub run_at: Instant,
    pub name: String,
    pub repeat: Option<Duration>,
}

impl Eq for Task {}
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.run_at == other.run_at
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse ordering for min-heap behavior
        other.run_at.cmp(&self.run_at)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Lightweight task scheduler for recovery + maintenance
pub struct Scheduler {
    queue: BinaryHeap<Task>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
        }
    }

    pub fn schedule(&mut self, task: Task) {
        self.queue.push(task);
    }

    pub fn tick(&mut self) -> Vec<Task> {
        let now = Instant::now();
        let mut ready = Vec::new();

        while let Some(task) = self.queue.peek() {
            if task.run_at <= now {
                ready.push(self.queue.pop().unwrap());
            } else {
                break;
            }
        }

        ready
    }
}
