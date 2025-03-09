use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Task {}

impl Task {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add(&self, task: Task) {
        println!("Added task: {:#?}", task);
    }
}
