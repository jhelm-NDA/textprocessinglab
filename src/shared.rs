// ============================================================
// Smart Pointers (Rc, RefCell, Arc, Mutex)
// ============================================================

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

pub struct SharedConfig {
    pub prefix: String,
}

pub fn build_shared_config() -> Rc<RefCell<SharedConfig>> {
    // TODO 9: Create Rc<RefCell<SharedConfig>>
    Rc::new(RefCell::new(SharedConfig { prefix: "".into() }))
}

pub fn build_threadsafe_counter() -> Arc<Mutex<i32>> {
    // TODO 10: Create Arc<Mutex<i32>>
    Arc::new(Mutex::new(0))
}
