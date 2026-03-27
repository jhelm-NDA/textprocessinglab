// ============================================================
// Lifetimes & Borrowed Structs
// ============================================================

pub struct BorrowedAnalyzer<'a> {
    // TODO 6: Store a borrowed &str with lifetime 'a
    text: &'a str,
}

impl<'a> BorrowedAnalyzer<'a> {
    pub fn new(text: &'a str) -> Self {
        // TODO 7: Initialize struct
        Self { text}
    }

    pub fn longest_word(&self) -> &'a str {
        // TODO 8: Return the longest word slice
        self.text
            .split_whitespace()
            .max_by_key(|w| w.len())
            .unwrap_or("")
        
    }
}
