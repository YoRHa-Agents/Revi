"""Tests for POST/GET /api/archive/{item_id:path}."""
import pytest


def test_archive_no_resolved_returns_400(client, plan_id):
    r = client.post(f"/api/archive/{plan_id}")
    assert r.status_code == 400


def test_archive_after_resolve_returns_200(client, plan_id):
    # Add and resolve a comment
    post_r = client.post(f"/api/comments/{plan_id}", json={
        "author": "Alice",
        "content": "To archive",
        "reference": {"type": "general", "value": None}
    })
    cid = post_r.json()["id"]
    client.patch(f"/api/comments/{plan_id}/{cid}/resolve")
    r = client.post(f"/api/archive/{plan_id}")
    assert r.status_code == 200


def test_archive_returns_batch_with_archived_at(client, plan_id):
    post_r = client.post(f"/api/comments/{plan_id}", json={
        "author": "Bob",
        "content": "Batch test",
        "reference": {"type": "general", "value": None}
    })
    cid = post_r.json()["id"]
    client.patch(f"/api/comments/{plan_id}/{cid}/resolve")
    r = client.post(f"/api/archive/{plan_id}")
    data = r.json()
    assert "archivedAt" in data
    assert data["archivedAt"] is not None


def test_archive_returns_batch_with_comments_array(client, plan_id):
    post_r = client.post(f"/api/comments/{plan_id}", json={
        "author": "Carol",
        "content": "In the batch",
        "reference": {"type": "general", "value": None}
    })
    cid = post_r.json()["id"]
    client.patch(f"/api/comments/{plan_id}/{cid}/resolve")
    r = client.post(f"/api/archive/{plan_id}")
    data = r.json()
    assert "comments" in data
    assert isinstance(data["comments"], list)
    assert len(data["comments"]) == 1


def test_archive_removes_resolved_from_active(client, plan_id):
    # Add 2 comments, resolve 1
    r1 = client.post(f"/api/comments/{plan_id}", json={
        "author": "Dave",
        "content": "Open comment",
        "reference": {"type": "general", "value": None}
    })
    r2 = client.post(f"/api/comments/{plan_id}", json={
        "author": "Eve",
        "content": "To resolve",
        "reference": {"type": "general", "value": None}
    })
    cid2 = r2.json()["id"]
    client.patch(f"/api/comments/{plan_id}/{cid2}/resolve")
    client.post(f"/api/archive/{plan_id}")
    # After archive, only 1 open comment remains
    active = client.get(f"/api/comments/{plan_id}").json()
    assert len(active) == 1
    assert active[0]["status"] == "open"


def test_get_archive_initially_empty(client, plan_id):
    r = client.get(f"/api/archive/{plan_id}")
    assert r.status_code == 200
    assert r.json() == []


def test_get_archive_shows_one_batch_after_archive(client, plan_id):
    post_r = client.post(f"/api/comments/{plan_id}", json={
        "author": "Frank",
        "content": "Batch 1",
        "reference": {"type": "general", "value": None}
    })
    cid = post_r.json()["id"]
    client.patch(f"/api/comments/{plan_id}/{cid}/resolve")
    client.post(f"/api/archive/{plan_id}")
    r = client.get(f"/api/archive/{plan_id}")
    assert r.status_code == 200
    batches = r.json()
    assert len(batches) == 1


def test_second_archive_creates_second_batch(client, plan_id):
    # First round
    r1 = client.post(f"/api/comments/{plan_id}", json={
        "author": "Grace",
        "content": "Round 1",
        "reference": {"type": "general", "value": None}
    })
    client.patch(f"/api/comments/{plan_id}/{r1.json()['id']}/resolve")
    client.post(f"/api/archive/{plan_id}")
    # Second round
    r2 = client.post(f"/api/comments/{plan_id}", json={
        "author": "Hank",
        "content": "Round 2",
        "reference": {"type": "general", "value": None}
    })
    client.patch(f"/api/comments/{plan_id}/{r2.json()['id']}/resolve")
    client.post(f"/api/archive/{plan_id}")
    r = client.get(f"/api/archive/{plan_id}")
    batches = r.json()
    assert len(batches) == 2
    # Newest batch should be first (index 0)
    assert batches[0]["archivedAt"] > batches[1]["archivedAt"]
