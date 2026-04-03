#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${1:-http://localhost:8000}"
REQUESTS="${REVI_LOAD_REQUESTS:-20}"
PARALLELISM="${REVI_LOAD_PARALLELISM:-4}"

seq "$REQUESTS" | xargs -I{} -P "$PARALLELISM" bash -lc '
  curl -fsS "'"$BASE_URL"'/api/reviews" >/dev/null
  curl -fsS "'"$BASE_URL"'/api/export/plans/sprint-1-design" >/dev/null
'

echo "load smoke passed against $BASE_URL with $REQUESTS iterations ($PARALLELISM parallel)"
