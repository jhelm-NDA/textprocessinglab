// ============================================================
// Traits & Implementations
// ============================================================

pub trait Processor {
    // TODO 1: Define process(&self, input: &str) -> String
    fn process(&self, input: &str) -> String;
}

pub struct Uppercase;

impl Processor for Uppercase {
    // TODO 2: Implement process() to return uppercase text
    fn process(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

pub struct WordCount;

impl Processor for WordCount {
    // TODO 3: Return "Count: X" where X is number of words
    fn process(&self, input: &str) -> String {
        let count = input.split_whitespace().count();
        format!("Count: {}", count)
    }
}
