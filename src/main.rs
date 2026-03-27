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
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let inputs = vec![
        "hello world".into(),
        "rust is amazing rust is fun".into(),
        "async pipelines rock".into(),
    ];

    // TODO 13: Run Uppercase pipeline
    let upper = Pipeline::new(Uppercase);
    let upper_out = upper.run(&inputs);
    show("Uppercase", &upper_out);

    // TODO 14: Run WordCount pipeline
    let wc = Pipeline::new(WordCount);
    let wc_out = wc.run(&inputs);
    show("Word Count", &wc_out);

    // TODO 15: Use BorrowedAnalyzer
    let analyzer = BorrowedAnalyzer::new(&inputs[2]);
    show("Longest Word", &[analyzer.longest_word().to_string()]);
    //println!("Longest word: {}", analyzer.longest_word());

    // TODO 16: Use Rc<RefCell<SharedConfig>>
    let cfg = build_shared_config();
    cfg.borrow_mut().prefix = "[CFG ".into();
    show("Config Prefix",&[cfg.borrow().prefix.clone()]);
    //println!("Config prefix: {}", cfg.borrow().prefix);

    // TODO 17: Use Arc<Mutex<i32>> with threads
    let counter = build_threadsafe_counter();
    let mut handles = vec![];
    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(std::thread::spawn(move || {
            let mut n = c.lock().unwrap();
            *n += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    //println!("Counter: {}", *counter.lock().unwrap());
    show("Counter", &[counter.lock().unwrap().to_string()]);

    // TODO 18: Run async processing
    let async_out = run_async(Uppercase, inputs.clone()).await;
    show("Async Uppercase", &async_out);

    // TODO 19: Compute word frequency
    let freq = word_frequency(&inputs);
    for (word, count) in freq {
        //println!("{}: {}", word, count);
        ui::show("Word Frequency Entry", &[format!("{}: {}", word, count)]);
    }
    
}
