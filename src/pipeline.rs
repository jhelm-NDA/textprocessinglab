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
        inputs
            .iter()
            .map(|s| self.processor.process(s))
            .collect()
    }
}

pub fn word_frequency(inputs: &[String]) -> HashMap<String, usize> {
    // TODO 5:
    // Build a HashMap<String, usize> of word counts

    let mut map = HashMap::new();

    for line in inputs {
        for word in line.split_whitespace() {
            let w = word.to_lowercase();
            *map.entry(w).or_insert(0) += 1;
        }
    }

    map
}
