# Storage Model

PTFS uses an object-based storage model.

## Object Structure

Each object contains:

- ID (u128)
- Data payload
- Metadata
- Checksum

## Allocation

- Extents-based allocation
- Variable block sizes (4KB → MB)
- Delayed allocation

## Tiering

Data is automatically moved between:

- RAM (hot)
- SSD (warm)
- HDD (cold)

## Pools

Multiple devices are combined into storage pools.

## Addressing

All addressing uses 128-bit space:

- Max file size: 2^128 bytes
- Max partition size: 2^128 bytes
