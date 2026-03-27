// ============================================================
// Threads vs Async
// ============================================================

use crate::processor::Processor;
use tokio::task;

pub async fn run_async<P: Processor + Send + Sync + 'static>(
    processor: P,
    inputs: Vec<String>,
) -> Vec<String> {
    let mut handles = vec![];

    for input in inputs {
        let p = &processor;

        // TODO 11:
        // Spawn async tasks to process each input
        // HINT: task::spawn(async move { p.process(&input) })
    }

    let mut results = vec![];

    // TODO 12:
    // Await each handle and push results

    results
}
