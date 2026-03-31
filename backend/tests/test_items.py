"""Tests for GET /api/reviews and GET /api/reviews/{item_id:path}."""
import pytest


def test_list_reviews_returns_three_items(client):
    r = client.get("/api/reviews")
    assert r.status_code == 200
    items = r.json()
    assert len(items) == 3


def test_list_reviews_item_types(client):
    r = client.get("/api/reviews")
    items = r.json()
    types = {i["id"]: i["type"] for i in items}
    assert types["plans/sprint-1-design"] == "plan"
    assert types["designs/ui-mockup-v1"] == "design"
    assert types["prototypes/review-flow"] == "prototype"


def test_list_reviews_initial_counts_are_zero(client):
    r = client.get("/api/reviews")
    items = r.json()
    for item in items:
        assert item["openCount"] == 0
        assert item["resolvedCount"] == 0


def test_list_reviews_item_has_correct_id_format(client):
    r = client.get("/api/reviews")
    items = r.json()
    ids = [i["id"] for i in items]
    assert "plans/sprint-1-design" in ids
    assert "designs/ui-mockup-v1" in ids
    assert "prototypes/review-flow" in ids


def test_get_review_plan_has_content_text(client, plan_id):
    r = client.get(f"/api/reviews/{plan_id}")
    assert r.status_code == 200
    data = r.json()
    assert data["contentText"] is not None
    assert "Sprint 1" in data["contentText"]


def test_get_review_design_has_null_content_text(client, design_id):
    r = client.get(f"/api/reviews/{design_id}")
    assert r.status_code == 200
    data = r.json()
    assert data["contentText"] is None


def test_get_review_nonexistent_returns_404(client):
    r = client.get("/api/reviews/nonexistent/item")
    assert r.status_code == 404


def test_get_review_detail_has_updated_at(client, plan_id):
    r = client.get(f"/api/reviews/{plan_id}")
    assert r.status_code == 200
    data = r.json()
    assert "updatedAt" in data
    assert data["updatedAt"] is not None


def test_open_count_increments_after_comment(client, plan_id):
    # Add a comment
    client.post(f"/api/comments/{plan_id}", json={
        "author": "Alice",
        "content": "Test comment",
        "reference": {"type": "general", "value": None}
    })
    r = client.get(f"/api/reviews/{plan_id}")
    assert r.status_code == 200
    data = r.json()
    assert data["openCount"] == 1
    assert data["resolvedCount"] == 0


def test_resolved_count_increments_after_resolve(client, plan_id):
    # Add and resolve a comment
    post_r = client.post(f"/api/comments/{plan_id}", json={
        "author": "Bob",
        "content": "Another comment",
        "reference": {"type": "general", "value": None}
    })
    cid = post_r.json()["id"]
    client.patch(f"/api/comments/{plan_id}/{cid}/resolve")
    r = client.get(f"/api/reviews/{plan_id}")
    data = r.json()
    assert data["openCount"] == 0
    assert data["resolvedCount"] == 1


def test_get_review_prototype_has_null_content_text(client, proto_id):
    r = client.get(f"/api/reviews/{proto_id}")
    assert r.status_code == 200
    data = r.json()
    assert data["contentText"] is None


def test_list_reviews_uses_camel_case_fields(client):
    r = client.get("/api/reviews")
    items = r.json()
    assert len(items) > 0
    item = items[0]
    # camelCase fields should be present
    assert "openCount" in item
    assert "resolvedCount" in item
    assert "updatedAt" in item
    # snake_case should NOT be present
    assert "open_count" not in item
    assert "resolved_count" not in item
    assert "updated_at" not in item
