"""Tests for GET /api/export/{item_id:path}."""
import pytest
from datetime import datetime


def test_export_schema_version(client, plan_id):
    r = client.get(f"/api/export/{plan_id}")
    assert r.status_code == 200
    data = r.json()
    assert data["schemaVersion"] == "1.0"


def test_export_summary_total_correct(client, plan_id):
    # Add 3 comments
    for i in range(3):
        client.post(f"/api/comments/{plan_id}", json={
            "author": f"User{i}",
            "content": f"Comment {i}",
            "reference": {"type": "general", "value": None}
        })
    r = client.get(f"/api/export/{plan_id}")
    data = r.json()
    assert data["summary"]["total"] == 3


def test_export_summary_open_correct(client, plan_id):
    # Add 2 comments, resolve 1
    r1 = client.post(f"/api/comments/{plan_id}", json={
        "author": "Alice",
        "content": "Open",
        "reference": {"type": "general", "value": None}
    })
    r2 = client.post(f"/api/comments/{plan_id}", json={
        "author": "Bob",
        "content": "To resolve",
        "reference": {"type": "general", "value": None}
    })
    client.patch(f"/api/comments/{plan_id}/{r2.json()['id']}/resolve")
    r = client.get(f"/api/export/{plan_id}")
    data = r.json()
    assert data["summary"]["open"] == 1


def test_export_summary_resolved_correct(client, plan_id):
    r1 = client.post(f"/api/comments/{plan_id}", json={
        "author": "Carol",
        "content": "Open",
        "reference": {"type": "general", "value": None}
    })
    r2 = client.post(f"/api/comments/{plan_id}", json={
        "author": "Dave",
        "content": "Resolved",
        "reference": {"type": "general", "value": None}
    })
    client.patch(f"/api/comments/{plan_id}/{r2.json()['id']}/resolve")
    r = client.get(f"/api/export/{plan_id}")
    data = r.json()
    assert data["summary"]["resolved"] == 1


def test_export_open_comments_only_open_status(client, plan_id):
    r1 = client.post(f"/api/comments/{plan_id}", json={
        "author": "Eve",
        "content": "Open",
        "reference": {"type": "general", "value": None}
    })
    r2 = client.post(f"/api/comments/{plan_id}", json={
        "author": "Frank",
        "content": "Resolved",
        "reference": {"type": "general", "value": None}
    })
    client.patch(f"/api/comments/{plan_id}/{r2.json()['id']}/resolve")
    r = client.get(f"/api/export/{plan_id}")
    data = r.json()
    open_ids = [c["id"] for c in data["openComments"]]
    assert r1.json()["id"] in open_ids
    assert r2.json()["id"] not in open_ids


def test_export_has_exported_at(client, plan_id):
    r = client.get(f"/api/export/{plan_id}")
    data = r.json()
    assert "exportedAt" in data
    assert data["exportedAt"] is not None
    # Should be a valid datetime string
    parsed = datetime.fromisoformat(data["exportedAt"].replace("Z", "+00:00"))
    assert parsed is not None


def test_export_all_resolved_open_comments_empty(client, plan_id):
    r1 = client.post(f"/api/comments/{plan_id}", json={
        "author": "Grace",
        "content": "To resolve",
        "reference": {"type": "general", "value": None}
    })
    cid = r1.json()["id"]
    client.patch(f"/api/comments/{plan_id}/{cid}/resolve")
    r = client.get(f"/api/export/{plan_id}")
    data = r.json()
    assert data["openComments"] == []


def test_export_nonexistent_item_returns_404(client):
    r = client.get("/api/export/nonexistent/item")
    assert r.status_code == 404


def test_export_open_comment_fields(client, plan_id):
    client.post(f"/api/comments/{plan_id}", json={
        "author": "Hank",
        "content": "Check fields",
        "reference": {"type": "section", "value": "## Overview"}
    })
    r = client.get(f"/api/export/{plan_id}")
    data = r.json()
    assert len(data["openComments"]) == 1
    comment = data["openComments"][0]
    assert "id" in comment
    assert "author" in comment
    assert "content" in comment
    assert "createdAt" in comment
    assert comment["author"] == "Hank"
    assert comment["content"] == "Check fields"


def test_export_empty_initially(client, plan_id):
    r = client.get(f"/api/export/{plan_id}")
    data = r.json()
    assert data["summary"]["total"] == 0
    assert data["summary"]["open"] == 0
    assert data["summary"]["resolved"] == 0
    assert data["openComments"] == []
