#!/bin/bash

echo "Running PTFS tests..."

cargo test --all

if [ $? -eq 0 ]; then
    echo "All tests passed."
else
    echo "Tests failed."
    exit 1
fi
