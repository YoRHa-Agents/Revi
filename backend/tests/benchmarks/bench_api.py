"""Benchmark tests for the Revi FastAPI backend.

Run with:
    python -m pytest backend/tests/benchmarks/ --benchmark-min-rounds=20 --benchmark-sort=mean -v
"""
import pytest
from fastapi.testclient import TestClient


@pytest.fixture(scope="module")
def bench_client(tmp_path_factory):
    """Module-scoped client for benchmarks — faster than per-test setup."""
    import backend.config as cfg_module
    import backend.storage as storage_module
    from backend.storage import CommentStore, ArchiveStore

    tmp = tmp_path_factory.mktemp("bench")
    workspace = tmp / "workspace"
    data = tmp / "data"
    workspace.mkdir()
    data.mkdir()
    for sub in ["plans", "designs", "prototypes"]:
        (workspace / sub).mkdir()
    (workspace / "plans" / "sprint-1-design.md").write_text(
        "# Sprint 1\n\n## Overview\n\n## Storage\n\n"
    )

    # Save originals so we can restore after the module
    orig_workspace = cfg_module.settings.workspace_path
    orig_data = cfg_module.settings.data_path
    orig_comment_store = storage_module.comment_store
    orig_archive_store = storage_module.archive_store

    import backend.routers.items as r_items
    import backend.routers.comments as r_comments
    import backend.routers.archive as r_archive
    import backend.routers.export as r_export
    orig_r_items_cs = r_items.comment_store
    orig_r_comments_cs = r_comments.comment_store
    orig_r_archive_cs = r_archive.comment_store
    orig_r_archive_as = r_archive.archive_store
    orig_r_export_cs = r_export.comment_store

    # Patch
    cfg_module.settings.workspace_path = workspace
    cfg_module.settings.data_path = data
    new_cs = CommentStore(data)
    new_as = ArchiveStore(data)
    storage_module.comment_store = new_cs
    storage_module.archive_store = new_as
    r_items.comment_store = new_cs
    r_comments.comment_store = new_cs
    r_archive.comment_store = new_cs
    r_archive.archive_store = new_as
    r_export.comment_store = new_cs

    from backend.main import app
    with TestClient(app) as c:
        yield c

    # Restore originals
    cfg_module.settings.workspace_path = orig_workspace
    cfg_module.settings.data_path = orig_data
    storage_module.comment_store = orig_comment_store
    storage_module.archive_store = orig_archive_store
    r_items.comment_store = orig_r_items_cs
    r_comments.comment_store = orig_r_comments_cs
    r_archive.comment_store = orig_r_archive_cs
    r_archive.archive_store = orig_r_archive_as
    r_export.comment_store = orig_r_export_cs


def test_bench_list_reviews(benchmark, bench_client):
    """GET /api/reviews — target < 10ms mean."""
    result = benchmark(bench_client.get, "/api/reviews")
    assert result.status_code == 200


def test_bench_get_review_detail(benchmark, bench_client):
    """GET /api/reviews/{item_id} — target < 5ms mean."""
    result = benchmark(bench_client.get, "/api/reviews/plans/sprint-1-design")
    assert result.status_code == 200


def test_bench_add_comment(benchmark, bench_client):
    """POST /api/comments/{item_id} — target < 20ms mean (includes disk write)."""
    payload = {
        "author": "bench",
        "content": "benchmark comment",
        "reference": {"type": "general", "value": None}
    }
    result = benchmark(bench_client.post, "/api/comments/plans/sprint-1-design", json=payload)
    assert result.status_code == 201


def test_bench_get_export(benchmark, bench_client):
    """GET /api/export/{item_id} — target < 5ms mean."""
    result = benchmark(bench_client.get, "/api/export/plans/sprint-1-design")
    assert result.status_code == 200
