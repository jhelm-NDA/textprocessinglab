// ============================================================
// Challenge E: Async File Pipeline
// ============================================================

use tokio::fs;
use tokio::task;
use crate::processor::Processor;

pub async fn process_files<P: Processor + Send + Sync + 'static>(
    processor: P,
    paths: Vec<String>,
) -> Vec<String> {
    let mut handles = vec![];

    for path in paths {
        let p = &processor;

        // TODO E1: Read file asynchronously and process contents
    }

    let mut results = vec![];

    // TODO E2: Await all tasks

    results
}
