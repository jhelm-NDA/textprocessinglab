// ============================================================
// Challenge C: Processor Graph (Rc<RefCell<Node>>)
// ============================================================

use std::rc::Rc;
use std::cell::RefCell;
use crate::processor::Processor;

pub struct Node {
    // TODO C1: Store processor and children
}

impl Node {
    pub fn new(processor: Box<dyn Processor>) -> Rc<Self> {
        // TODO C2: Initialize node
        Rc::new(Node { /* fill */ })
    }

    pub fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
        // TODO C3: Add child to parent
    }

    pub fn run(&self, input: &str) -> Vec<String> {
        // TODO C4: Traverse graph and collect results
        vec![]
    }
}
