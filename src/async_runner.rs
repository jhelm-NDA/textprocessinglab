// ============================================================
// Threads vs Async
// ============================================================

use crate::processor::Processor;
use tokio::task;
use std::sync::Arc;

pub async fn run_async<P: Processor + Send + Sync + 'static>(
    processor: P,
    inputs: Vec<String>,
) -> Vec<String> {
    let processor = Arc::new(processor);
    let mut handles = vec![];

    for input in inputs {
        //let p = &processor;
        let p = Arc::clone(&processor); 

        // TODO 11:
        // Spawn async tasks to process each input
        // HINT: task::spawn(async move { p.process(&input) })
        handles.push(task::spawn(async move {
            p.process(&input)
        }));
    }

    let mut results = vec![];

    // TODO 12:
    // Await each handle and push results
    for h in handles {
        results.push(h.await.unwrap());
    }

    results
}
