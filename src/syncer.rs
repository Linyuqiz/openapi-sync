use crate::task::Task;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Syncer {}

impl Syncer {
    pub fn new() -> Syncer {
        Syncer::default()
    }

    pub fn add_task(&self, task: Task) {
        println!("adding sync task");
    }

    pub fn start(&self) {
        println!("Syncer started");
    }
}
