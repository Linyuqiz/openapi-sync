pub mod syncer;
pub mod task;

#[cfg(test)]
mod tests {
    use crate::syncer::Syncer;
    use crate::task::Task;

    #[test]
    fn test_syncer() {
        let syncer = Syncer::new();
        syncer.add_task(Task::new());
        syncer.start()
    }
}
