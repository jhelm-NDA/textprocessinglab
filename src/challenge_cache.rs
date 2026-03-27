// ============================================================
// Challenge D: Shared Async Cache
// ============================================================

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task;

pub async fn cached_process<F>(
    inputs: Vec<String>,
    cache: Arc<Mutex<HashMap<String, String>>>,
    processor: F,
) -> Vec<String>
where
    F: Fn(&str) -> String + Send + Sync + 'static,
{
    let mut handles = vec![];

    for input in inputs {
        let cache = Arc::clone(&cache);
        let p = &processor;

        // TODO D1: Check cache, compute if missing, insert result
    }

    let mut results = vec![];

    // TODO D2: Await all tasks

    results
}
