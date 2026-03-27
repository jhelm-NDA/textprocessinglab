# textprocessinglab

Goal
Build a modular, extensible text‑processing engine that evolves across multiple Rust concepts.
You will complete TODOs across several modules, each representing a major Rust topic.

Project Structure
src/
  main.rs
  processor.rs
  pipeline.rs
  borrowed.rs
  shared.rs
  async_runner.rs
  ui.rs
  challenge_dyn_pipeline.rs
  challenge_async_trait.rs
  challenge_graph.rs
  challenge_cache.rs
  challenge_file_pipeline.rs

Part 1 — Traits & Implementations (processor.rs)
//TODO 1
Define a trait Processor with:
fn process(&self, input: &str) -> String

//TODO 2
Implement Processor for Uppercase.

//TODO 3
Implement Processor for WordCount.

Part 2 — Generic Pipeline (pipeline.rs)
//TODO 4
Implement Pipeline<P>::run() using iterator adaptors.

//TODO 5
Implement word_frequency() using a HashMap.

Part 3 — Lifetimes (borrowed.rs)
//TODO 6
Create a struct that stores a borrowed &str.

//TODO 7
Implement new().

//TODO 8
Implement longest_word().

Part 4 — Smart Pointers (shared.rs)
//TODO 9
Create a shared config using Rc<RefCell<_>>.

//TODO 10
Create a thread‑safe counter using Arc<Mutex<_>>.

Part 5 — Async (async_runner.rs)
//TODO 11
Spawn async tasks using tokio::spawn.

//TODO 12
Await each task and collect results.

Part 6 — Integration (main.rs)
//TODO 13
Run the Uppercase pipeline.

//TODO 14
Run the WordCount pipeline.

//TODO 15
Use BorrowedAnalyzer.

//TODO 16
Modify shared config.

//TODO 17
Use thread‑safe counter with threads.

//TODO 18
Run async processing.

//TODO 19
Compute word frequency.

Optional Challenge Modules

Challenge A — Dynamic Trait Objects
Use Box<dyn Processor> instead of generics.

Challenge B — Async Trait Processors
Define an async trait using async_trait.

Challenge C — Processor Graph
Use Rc<RefCell<Node>> to build a processor graph.

Challenge D — Shared Async Cache
Use Arc<Mutex<HashMap>> to cache results across tasks.

Challenge E — Async File Pipeline
Read files concurrently and process them.
