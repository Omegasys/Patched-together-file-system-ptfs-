# PTFS Architecture

PTFS is designed as a modular, object-based filesystem with strong data integrity guarantees.

## Layers

1. API Layer (FUSE / CLI / bindings)
2. Core Filesystem Logic
3. Storage Services (allocator, cache, logging)
4. Storage Backends + RAID
5. Physical Devices

## Key Concepts

- Object-based storage instead of raw blocks
- Copy-on-write (CoW) for all writes
- Log-structured zones for SSD optimization
- Integrated volume management

## Design Goals

- Scalability (128-bit addressing)
- Reliability (Merkle trees + checksums)
- Flexibility (pluggable storage backends)
