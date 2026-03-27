// ============================================================
// Generics, Iterator Adaptors, HashMaps
// ============================================================

use crate::processor::Processor;
use std::collections::HashMap;

pub struct Pipeline<P: Processor> {
    processor: P,
}

impl<P: Processor> Pipeline<P> {
    pub fn new(processor: P) -> Self {
        Self { processor }
    }

    pub fn run(&self, inputs: &[String]) -> Vec<String> {
        // TODO 4:
        // Use iterator adaptors to:
        //   - iterate over inputs
        //   - call processor.process()
        //   - collect results into Vec<String>
        vec![]
    }
}

pub fn word_frequency(inputs: &[String]) -> HashMap<String, usize> {
    // TODO 5:
    // Build a HashMap<String, usize> of word counts
    HashMap::new()
}
