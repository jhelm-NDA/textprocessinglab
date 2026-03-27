// ============================================================
// Challenge B: Async Trait Processors
// ============================================================

use async_trait::async_trait;

#[async_trait]
pub trait AsyncProcessor {
    // TODO B1: Define async fn process_async(&self, input: &str) -> String
}

pub struct AsyncUpper;

#[async_trait]
impl AsyncProcessor for AsyncUpper {
    // TODO B2: Implement async processor
}
