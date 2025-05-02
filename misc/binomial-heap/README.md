# Binomial Heap (Rust)

This is a minimal, proof-of-concept implementation of a [binomial heap](https://en.wikipedia.org/wiki/Binomial_heap) in Rust. A binomial heap is a priority queue data structure that supports efficient merging of heaps (meld) along with the usual enqueue, extract-min, and find-min operations.

## Project Structure

- `src/bheap.rs` — The core binomial heap implementation  
- `tests/test_bheap.rs` — Unit tests validating the heap behavior

## Running the Tests

To run all tests, use:

```bash
cargo test
```


## Requirements

- Rust (via [`rustup`](https://rustup.rs))  
- Cargo (comes with Rust)