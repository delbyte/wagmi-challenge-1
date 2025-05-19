# WAGMI-9000

Optimized Rust-based REST API for WAGMI Challenge 001: Project WAGMI-9000.

## Overview
Implements `POST /wagmi` with:
- Ping: `{"message": "wagmi", "timestamp": "...", "lang": "Rust"}`.
- Addition: Validates `a`, `b` (non-negative, sum <= 100).
- Error: `{"error": "Invalid input"}`.

Optimizations:
- `sonic-rs` for JSON serialization and deserialization.
- Cached timestamps (1ms).
- Single worker, 6,000 connections.

## Setup
1. **Clone**:
   ```bash
   git clone https://github.com/delbyte/wagmi-challenge-1.git
   cd wagmi-9000