# Revi Load Testing Guide

## Overview

Revi no longer keeps a Python load-test stack. The supported performance checks
now use:

- `cargo bench --manifest-path server/Cargo.toml` for in-process Rust API
  benchmarks.
- `bash scripts/load-smoke.sh` for concurrent real-server smoke traffic against
  the running Rust backend.

## Prerequisites

```bash
# Start the supported Rust backend first
cd /path/to/Revi
cargo run --manifest-path server/Cargo.toml --bin revi -- --port 8000
```

## Run Commands

### API micro-benchmarks
```bash
cd server
cargo bench
```

### Concurrent smoke load
```bash
bash scripts/load-smoke.sh
```

Optional tuning:

```bash
REVI_LOAD_REQUESTS=100 REVI_LOAD_PARALLELISM=8 bash scripts/load-smoke.sh http://localhost:8000
```

## Targets

- Smoke load should finish without HTTP failures.
- `cargo bench` should stay stable across runs for `GET /api/reviews`,
  `GET /api/reviews/{item_id}`, `POST /api/comments/{item_id}`, and
  `GET /api/export/{item_id}`.

## Notes

- `scripts/load-smoke.sh` targets the real Rust server, not the in-memory test
  harness.
- `cargo bench` measures handler-level performance without requiring any Python
  tooling.
- For longer runs, monitor memory with `top` or `htop` while the Rust backend is
  serving requests.
