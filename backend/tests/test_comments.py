"""Tests for GET/POST /api/comments/{item_id:path} and PATCH .../resolve."""
import pytest


def test_list_comments_empty_initially(client, plan_id):
    r = client.get(f"/api/comments/{plan_id}")
    assert r.status_code == 200
    assert r.json() == []


def test_add_comment_returns_201(client, plan_id):
    r = client.post(f"/api/comments/{plan_id}", json={
        "author": "Alice",
        "content": "Looks good",
        "reference": {"type": "general", "value": None}
    })
    assert r.status_code == 201


def test_add_comment_returns_uuid_id(client, plan_id):
    r = client.post(f"/api/comments/{plan_id}", json={
        "author": "Alice",
        "content": "Test",
        "reference": {"type": "general", "value": None}
    })
    data = r.json()
    assert "id" in data
    assert len(data["id"]) == 36  # UUID4 length
    assert data["id"].count("-") == 4


def test_new_comment_has_open_status(client, plan_id):
    r = client.post(f"/api/comments/{plan_id}", json={
        "author": "Bob",
        "content": "Feedback",
        "reference": {"type": "general", "value": None}
    })
    data = r.json()
    assert data["status"] == "open"


def test_new_comment_has_null_resolved_at(client, plan_id):
    r = client.post(f"/api/comments/{plan_id}", json={
        "author": "Carol",
        "content": "Note",
        "reference": {"type": "general", "value": None}
    })
    data = r.json()
    assert data["resolvedAt"] is None


def test_list_comments_after_post(client, plan_id):
    client.post(f"/api/comments/{plan_id}", json={
        "author": "Dave",
        "content": "Comment 1",
        "reference": {"type": "general", "value": None}
    })
    r = client.get(f"/api/comments/{plan_id}")
    assert r.status_code == 200
    assert len(r.json()) == 1


def test_resolve_comment_sets_resolved_status(client, plan_id):
    post_r = client.post(f"/api/comments/{plan_id}", json={
        "author": "Eve",
        "content": "To be resolved",
        "reference": {"type": "general", "value": None}
    })
    cid = post_r.json()["id"]
    patch_r = client.patch(f"/api/comments/{plan_id}/{cid}/resolve")
    assert patch_r.status_code == 200
    data = patch_r.json()
    assert data["status"] == "resolved"


def test_resolve_comment_sets_resolved_at(client, plan_id):
    post_r = client.post(f"/api/comments/{plan_id}", json={
        "author": "Frank",
        "content": "Resolution check",
        "reference": {"type": "general", "value": None}
    })
    cid = post_r.json()["id"]
    patch_r = client.patch(f"/api/comments/{plan_id}/{cid}/resolve")
    data = patch_r.json()
    assert data["resolvedAt"] is not None


def test_resolve_missing_comment_returns_404(client, plan_id):
    r = client.patch(f"/api/comments/{plan_id}/nonexistent-uuid/resolve")
    assert r.status_code == 404


def test_post_comment_nonexistent_item_returns_404(client):
    r = client.post("/api/comments/plans/nonexistent", json={
        "author": "Ghost",
        "content": "To the void",
        "reference": {"type": "general", "value": None}
    })
    assert r.status_code == 404


def test_post_comment_missing_author_returns_422(client, plan_id):
    r = client.post(f"/api/comments/{plan_id}", json={
        "content": "No author here",
        "reference": {"type": "general", "value": None}
    })
    assert r.status_code == 422


def test_post_comment_missing_content_returns_422(client, plan_id):
    r = client.post(f"/api/comments/{plan_id}", json={
        "author": "Someone",
        "reference": {"type": "general", "value": None}
    })
    assert r.status_code == 422


def test_multiple_comments_count(client, plan_id):
    for i in range(3):
        client.post(f"/api/comments/{plan_id}", json={
            "author": f"User{i}",
            "content": f"Comment {i}",
            "reference": {"type": "general", "value": None}
        })
    r = client.get(f"/api/comments/{plan_id}")
    assert r.status_code == 200
    assert len(r.json()) == 3


def test_comment_persistence(client, plan_id):
    # Create a comment via POST
    post_r = client.post(f"/api/comments/{plan_id}", json={
        "author": "Persistent",
        "content": "Saved data",
        "reference": {"type": "general", "value": None}
    })
    created_id = post_r.json()["id"]
    # Read it back via GET
    get_r = client.get(f"/api/comments/{plan_id}")
    comments = get_r.json()
    ids = [c["id"] for c in comments]
    assert created_id in ids


def test_comment_has_camel_case_fields(client, plan_id):
    r = client.post(f"/api/comments/{plan_id}", json={
        "author": "CamelTest",
        "content": "Check fields",
        "reference": {"type": "general", "value": None}
    })
    data = r.json()
    # camelCase should be present
    assert "createdAt" in data
    assert "resolvedAt" in data
    assert "itemId" in data
    # snake_case should NOT be present
    assert "created_at" not in data
    assert "resolved_at" not in data
    assert "item_id" not in data


def test_comment_item_id_field(client, plan_id):
    r = client.post(f"/api/comments/{plan_id}", json={
        "author": "IdCheck",
        "content": "Check itemId",
        "reference": {"type": "general", "value": None}
    })
    data = r.json()
    assert data["itemId"] == plan_id
