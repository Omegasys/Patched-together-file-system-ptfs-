# Merkle Tree Integrity

PTFS uses Merkle trees for full data verification.

## Structure

- Leaves: data block hashes
- Internal nodes: hash of children
- Root: global integrity hash

## Benefits

- Detects corruption (bit rot)
- Enables efficient verification
- Supports deduplication

## Algorithm

- Default: BLAKE3

## Self-Healing

If corruption is detected:

1. Compare replicas
2. Select valid copy
3. Repair corrupted data
