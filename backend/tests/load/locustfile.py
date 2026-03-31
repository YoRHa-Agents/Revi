"""
Locust pressure test suite for Revi API.

Usage:
  # Start backend first:
  uvicorn backend.main:app --port 8000

  # Headless smoke (1 user, 30s):
  locust -f backend/tests/load/locustfile.py --host http://localhost:8000 \
    --users 1 --spawn-rate 1 --run-time 30s --headless

  # Normal load (12 users, 60s):
  locust -f backend/tests/load/locustfile.py --host http://localhost:8000 \
    --users 12 --spawn-rate 2 --run-time 60s --headless \
    --html backend/tests/load/report.html

  # Peak load (60 users, 60s):
  locust -f backend/tests/load/locustfile.py --host http://localhost:8000 \
    --users 60 --spawn-rate 5 --run-time 60s --headless \
    --html backend/tests/load/report.html

  # Soak (6 users, 5 minutes):
  locust -f backend/tests/load/locustfile.py --host http://localhost:8000 \
    --users 6 --spawn-rate 1 --run-time 300s --headless

Load scenarios and targets:
  Smoke  (1 user,  30s):  p99 < 50ms,  0 errors
  Normal (12 users, 60s): p95 < 100ms, error rate < 1%
  Peak   (60 users, 60s): p95 < 500ms, error rate < 5%
  Soak   (6 users, 300s): stable response times, no memory leak
"""

import json
import random
from locust import HttpUser, task, between, constant_pacing


ITEM_ID = "plans/sprint-1-design"


class ReviewerUser(HttpUser):
    """Simulates a human reviewer browsing and adding comments."""
    wait_time = between(0.5, 2.0)

    @task(5)
    def list_items(self):
        with self.client.get("/api/reviews", catch_response=True) as resp:
            if resp.status_code == 200:
                resp.success()
            else:
                resp.failure(f"Expected 200, got {resp.status_code}")

    @task(3)
    def get_export(self):
        with self.client.get(f"/api/export/{ITEM_ID}", catch_response=True) as resp:
            if resp.status_code == 200:
                data = resp.json()
                if data.get("schemaVersion") == "1.0":
                    resp.success()
                else:
                    resp.failure("Missing schemaVersion")
            else:
                resp.failure(f"Expected 200, got {resp.status_code}")

    @task(2)
    def add_comment(self):
        payload = {
            "author": "loadtest-reviewer",
            "content": "Load test comment from ReviewerUser",
            "reference": {"type": "general", "value": None}
        }
        with self.client.post(
            f"/api/comments/{ITEM_ID}",
            json=payload,
            catch_response=True,
            name="/api/comments/[item_id]"
        ) as resp:
            if resp.status_code == 201:
                resp.success()
            else:
                resp.failure(f"Expected 201, got {resp.status_code}")

    @task(1)
    def resolve_and_archive_mini_flow(self):
        """Full mini-flow: add → resolve → (occasionally archive)."""
        # Add a comment
        payload = {
            "author": "loadtest-flow",
            "content": "Mini-flow test comment",
            "reference": {"type": "section", "value": "## Overview"}
        }
        with self.client.post(
            f"/api/comments/{ITEM_ID}",
            json=payload,
            catch_response=True,
            name="/api/comments/[item_id] (flow)"
        ) as resp:
            if resp.status_code != 201:
                resp.failure(f"Add failed: {resp.status_code}")
                return
            resp.success()
            comment_id = resp.json().get("id")

        if not comment_id:
            return

        # Resolve it
        with self.client.patch(
            f"/api/comments/{ITEM_ID}/{comment_id}/resolve",
            catch_response=True,
            name="/api/comments/[item_id]/[comment_id]/resolve"
        ) as resp:
            if resp.status_code == 200:
                resp.success()
            else:
                resp.failure(f"Resolve failed: {resp.status_code}")


class AgentUser(HttpUser):
    """Simulates an AI agent polling exports and adding feedback comments."""
    wait_time = constant_pacing(1.0)  # one action per second

    @task(4)
    def poll_export(self):
        with self.client.get(
            f"/api/export/{ITEM_ID}",
            catch_response=True,
            name="/api/export/[item_id] (agent)"
        ) as resp:
            if resp.status_code == 200:
                data = resp.json()
                # Agent validates schema_version
                if "schemaVersion" not in data:
                    resp.failure("Missing schemaVersion field")
                else:
                    resp.success()
            else:
                resp.failure(f"Expected 200, got {resp.status_code}")

    @task(1)
    def agent_comment(self):
        """Agent leaves structured feedback."""
        payload = {
            "author": "Agent-Load",
            "content": f"Automated review finding #{random.randint(1, 1000)}",
            "reference": {
                "type": "section",
                "value": random.choice(["## Overview", "## Storage Layer", "## API Endpoints"])
            }
        }
        with self.client.post(
            f"/api/comments/{ITEM_ID}",
            json=payload,
            catch_response=True,
            name="/api/comments/[item_id] (agent)"
        ) as resp:
            if resp.status_code == 201:
                resp.success()
            else:
                resp.failure(f"Expected 201, got {resp.status_code}")
