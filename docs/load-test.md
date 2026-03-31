# Revi Load Testing Guide

## Overview

Revi uses [Locust](https://locust.io) for HTTP pressure testing. Two user classes simulate realistic traffic:

- **ReviewerUser** — Human reviewer: list items, get exports, add comments, mini resolve-flows
- **AgentUser** — AI agent: poll exports, add structured feedback

## Prerequisites

```bash
pip install locust>=2.28.0
# Start the backend first
cd /path/to/Revi
uvicorn backend.main:app --port 8000
```

## Load Scenarios

| Scenario | Users | Spawn Rate | Duration | p95 Target | Error Target |
|----------|-------|-----------|----------|-----------|-------------|
| Smoke    | 1 ReviewerUser | 1/s | 30s | < 50ms | 0% |
| Normal   | 10 ReviewerUser + 2 AgentUser | 2/s | 60s | < 100ms | < 1% |
| Peak     | 50 ReviewerUser + 10 AgentUser | 5/s | 60s | < 500ms | < 5% |
| Soak     | 5 ReviewerUser + 1 AgentUser | 1/s | 300s | stable | < 1% |

## Run Commands

### Smoke test (headless)
```bash
locust -f backend/tests/load/locustfile.py \
  --host http://localhost:8000 \
  --users 1 --spawn-rate 1 --run-time 30s --headless
```

### Normal load (headless, with HTML report)
```bash
locust -f backend/tests/load/locustfile.py \
  --host http://localhost:8000 \
  --users 12 --spawn-rate 2 --run-time 60s --headless \
  --html backend/tests/load/report.html
```

### Peak load
```bash
locust -f backend/tests/load/locustfile.py \
  --host http://localhost:8000 \
  --users 60 --spawn-rate 5 --run-time 60s --headless \
  --html backend/tests/load/report.html
```

### Soak test
```bash
locust -f backend/tests/load/locustfile.py \
  --host http://localhost:8000 \
  --users 6 --spawn-rate 1 --run-time 300s --headless
```

### Interactive UI (opens browser at http://localhost:8089)
```bash
locust -f backend/tests/load/locustfile.py --host http://localhost:8000
```

## Reading Results

The headless output shows:
- **RPS** — requests per second
- **p50/p95/p99** — latency percentiles in ms
- **Failures** — failed request count and rate

Targets are met when:
- Normal scenario: p95 < 100ms, failure rate < 1%
- Peak scenario: p95 < 500ms, failure rate < 5%

## Notes

- The locustfile targets the real running server (not TestClient) — start backend before running
- Comments accumulate during load tests; the backend uses an in-process cache so performance stays stable
- For soak tests, monitor memory with `top` or `htop` — stable memory = no leak
