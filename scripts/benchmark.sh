#!/bin/bash

echo "Running PTFS benchmarks..."

# Run examples as simple benchmarks for now
cargo run --example simple_fs
cargo run --example snapshot_demo

echo "Benchmark complete (basic)."
