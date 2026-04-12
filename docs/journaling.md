# Journaling & Log Structure

PTFS combines journaling with log-structured design.

## Journaling

- Write-ahead logging (WAL)
- Ensures atomic transactions

## Log-Structured Storage

- Sequential writes for SSD efficiency
- Reduced fragmentation

## Crash Recovery

- Replay journal
- Restore last consistent state

## Hybrid Model

- CoW is primary
- Journal acts as fallback
