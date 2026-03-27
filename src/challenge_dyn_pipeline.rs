// ============================================================
// Challenge A: Dynamic Trait Objects
// ============================================================

use crate::processor::Processor;

pub struct DynPipeline {
    processor: Box<dyn Processor>,
}

impl DynPipeline {
    pub fn new(processor: Box<dyn Processor>) -> Self {
        Self { processor }
    }

    pub fn run(&self, inputs: &[String]) -> Vec<String> {
        // TODO A1: Implement dynamic pipeline run()
        vec![]
    }
}
