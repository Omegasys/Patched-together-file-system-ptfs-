# PTFS API

## Goals

- POSIX compatibility
- Extended advanced features

## Core Operations

- open()
- read()
- write()
- sync()
- snapshot()

## Extended Features

- Snapshot management
- Deduplication control
- Compression settings
- Encryption policies

## Interfaces

- FUSE (user-space)
- Native kernel module (future)
- Rust API bindings

## Example (Rust)

```rust
use ptfs_api::Filesystem;

let fs = Filesystem::open("/mnt/ptfs");
fs.write("file.txt", b"hello world")?;
