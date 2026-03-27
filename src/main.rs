// ============================================================
// Integration
// ============================================================

mod processor;
mod pipeline;
mod borrowed;
mod shared;
mod async_runner;
mod ui;

use processor::*;
use pipeline::*;
use borrowed::*;
use shared::*;
use async_runner::*;
use ui::*;

#[tokio::main]
async fn main() {
    let inputs = vec![
        "hello world".into(),
        "rust is amazing".into(),
        "async pipelines rock".into(),
    ];

    // TODO 13: Run Uppercase pipeline

    // TODO 14: Run WordCount pipeline

    // TODO 15: Use BorrowedAnalyzer

    // TODO 16: Use Rc<RefCell<SharedConfig>>

    // TODO 17: Use Arc<Mutex<i32>> with threads

    // TODO 18: Run async processing

    // TODO 19: Compute word frequency
}
