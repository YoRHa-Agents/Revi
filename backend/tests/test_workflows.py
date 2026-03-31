"""Full-scene workflow tests crossing multiple endpoints."""
import pytest


def test_human_review_full_loop(client):
    item_id = "plans/sprint-1-design"

    # Verify item appears in list
    items = client.get("/api/reviews").json()
    assert any(i["id"] == item_id for i in items)

    # Add 3 comments
    for i in range(3):
        r = client.post(f"/api/comments/{item_id}", json={
            "author": f"User{i}", "content": f"Comment {i}",
            "reference": {"type": "general", "value": None}
        })
        assert r.status_code == 201

    # Verify open count = 3
    detail = client.get(f"/api/reviews/{item_id}").json()
    assert detail["openCount"] == 3

    # Resolve 2 comments
    comments = client.get(f"/api/comments/{item_id}").json()
    for c in comments[:2]:
        r = client.patch(f"/api/comments/{item_id}/{c['id']}/resolve")
        assert r.status_code == 200

    # Verify resolved count = 2
    detail = client.get(f"/api/reviews/{item_id}").json()
    assert detail["resolvedCount"] == 2

    # Archive resolved
    r = client.post(f"/api/archive/{item_id}")
    assert r.status_code == 200

    # Verify only 1 comment remains (open)
    active = client.get(f"/api/comments/{item_id}").json()
    assert len(active) == 1
    assert active[0]["status"] == "open"

    # Archive has 1 batch with 2 comments
    archive = client.get(f"/api/archive/{item_id}").json()
    assert len(archive) == 1
    assert len(archive[0]["comments"]) == 2

    # Export shows 1 open comment
    export = client.get(f"/api/export/{item_id}").json()
    assert export["summary"]["open"] == 1
    assert len(export["openComments"]) == 1

    # Add 2 more, resolve 2, archive again → 2 batches total
    comments = client.get(f"/api/comments/{item_id}").json()
    for c in comments:  # resolve the remaining 1 open
        client.patch(f"/api/comments/{item_id}/{c['id']}/resolve")
    for i in range(2):
        r = client.post(f"/api/comments/{item_id}", json={
            "author": "User", "content": f"New {i}",
            "reference": {"type": "section", "value": "## Overview"}
        })
        assert r.status_code == 201
    # Resolve those 2 new ones
    new_comments = [c for c in client.get(f"/api/comments/{item_id}").json() if c["status"] == "open"]
    for c in new_comments[:2]:
        client.patch(f"/api/comments/{item_id}/{c['id']}/resolve")
    r = client.post(f"/api/archive/{item_id}")
    assert r.status_code == 200
    archive = client.get(f"/api/archive/{item_id}").json()
    assert len(archive) == 2
    # Newest batch is first
    assert archive[0]["archivedAt"] > archive[1]["archivedAt"]


def test_agent_consume_flow(client):
    item_id = "plans/sprint-1-design"

    # Agent posts 5 comments
    comment_ids = []
    for i in range(5):
        r = client.post(f"/api/comments/{item_id}", json={
            "author": "Agent-Test",
            "content": f"Finding #{i}: needs attention",
            "reference": {"type": "section", "value": "## Overview"}
        })
        assert r.status_code == 201
        comment_ids.append(r.json()["id"])

    # Get export, verify 5 open
    export = client.get(f"/api/export/{item_id}").json()
    assert export["schemaVersion"] == "1.0"
    assert export["summary"]["open"] == 5
    assert len(export["openComments"]) == 5
    # Verify reference field is present
    for c in export["openComments"]:
        assert c["reference"]["type"] == "section"

    # Resolve all 5
    for cid in comment_ids:
        r = client.patch(f"/api/comments/{item_id}/{cid}/resolve")
        assert r.status_code == 200

    # Archive
    r = client.post(f"/api/archive/{item_id}")
    assert r.status_code == 200

    # Export now empty
    export = client.get(f"/api/export/{item_id}").json()
    assert export["openComments"] == []
    assert export["summary"]["open"] == 0

    # Archive has all 5
    archive = client.get(f"/api/archive/{item_id}").json()
    assert len(archive) == 1
    assert len(archive[0]["comments"]) == 5


def test_multi_item_independence(client):
    plan_id = "plans/sprint-1-design"
    design_id = "designs/ui-mockup-v1"

    # Add comment to plan
    r = client.post(f"/api/comments/{plan_id}", json={
        "author": "Alice", "content": "Plan comment",
        "reference": {"type": "general", "value": None}
    })
    plan_cid = r.json()["id"]

    # Add comment to design
    client.post(f"/api/comments/{design_id}", json={
        "author": "Bob", "content": "Design comment",
        "reference": {"type": "annotation", "value": "1"}
    })

    # Resolve plan comment and archive
    client.patch(f"/api/comments/{plan_id}/{plan_cid}/resolve")
    client.post(f"/api/archive/{plan_id}")

    # Design comments are unaffected
    design_comments = client.get(f"/api/comments/{design_id}").json()
    assert len(design_comments) == 1
    assert design_comments[0]["status"] == "open"

    # Plan export is empty, design export has 1
    plan_export = client.get(f"/api/export/{plan_id}").json()
    assert plan_export["summary"]["open"] == 0

    design_export = client.get(f"/api/export/{design_id}").json()
    assert design_export["summary"]["open"] == 1


def test_edge_cases(client):
    item_id = "plans/sprint-1-design"

    # Export before any comments
    export = client.get(f"/api/export/{item_id}").json()
    assert export["summary"]["total"] == 0
    assert export["summary"]["open"] == 0
    assert export["openComments"] == []

    # Double-archive (no resolved) → 400
    r = client.post(f"/api/archive/{item_id}")
    assert r.status_code == 400

    # Add and resolve comment
    r = client.post(f"/api/comments/{item_id}", json={
        "author": "Test", "content": "x",
        "reference": {"type": "general", "value": None}
    })
    cid = r.json()["id"]
    client.patch(f"/api/comments/{item_id}/{cid}/resolve")

    # Resolve already-resolved → still 200, resolvedAt unchanged
    first_resolve = client.patch(f"/api/comments/{item_id}/{cid}/resolve").json()
    second_resolve = client.patch(f"/api/comments/{item_id}/{cid}/resolve").json()
    assert first_resolve["resolvedAt"] == second_resolve["resolvedAt"]

    # Archive → 200
    r = client.post(f"/api/archive/{item_id}")
    assert r.status_code == 200

    # Double-archive again → 400
    r = client.post(f"/api/archive/{item_id}")
    assert r.status_code == 400


def test_upload_discover_comment_flow(client, tmp_dirs):
    workspace, data = tmp_dirs

    # Upload a new plan
    content = b"# My New Plan\n\n## Overview\nTest plan.\n"
    r = client.post("/api/upload", files={"file": ("my-new-plan.md", content, "text/markdown")})
    assert r.status_code == 201
    item_id = r.json()["itemId"]
    assert item_id == "plans/my-new-plan"

    # Discover it in list
    items = client.get("/api/reviews").json()
    ids = [i["id"] for i in items]
    assert "plans/my-new-plan" in ids

    # Add comment
    r = client.post(f"/api/comments/{item_id}", json={
        "author": "Tester", "content": "New plan comment",
        "reference": {"type": "section", "value": "## Overview"}
    })
    assert r.status_code == 201

    # Export has it
    export = client.get(f"/api/export/{item_id}").json()
    assert export["summary"]["open"] == 1
    assert export["openComments"][0]["content"] == "New plan comment"
