import pytest
from pathlib import Path
from fastapi.testclient import TestClient
import backend.config as cfg_module
import backend.storage as storage_module
from backend.storage import CommentStore, ArchiveStore, MetadataStore


@pytest.fixture
def tmp_dirs(tmp_path):
    workspace = tmp_path / "workspace"
    data = tmp_path / "data"
    workspace.mkdir()
    data.mkdir()
    for sub in ["plans", "designs", "prototypes"]:
        (workspace / sub).mkdir()
    (workspace / "plans" / "sprint-1-design.md").write_text(
        "# Sprint 1 — System Design\n\n## Overview\n\n## Storage Layer\nJSON comments.\n"
    )
    (workspace / "designs" / "ui-mockup-v1.png").write_bytes(
        b'\x89PNG\r\n\x1a\n' + b'\x00' * 30
    )
    (workspace / "prototypes" / "review-flow.html").write_text(
        "<html><body><h1>Proto</h1></body></html>"
    )
    return workspace, data


@pytest.fixture
def client(tmp_dirs, monkeypatch):
    workspace, data = tmp_dirs
    # Patch settings
    monkeypatch.setattr(cfg_module.settings, "workspace_path", workspace)
    monkeypatch.setattr(cfg_module.settings, "data_path", data)
    # Re-initialize the module-level singletons with new paths
    new_comment_store = CommentStore(data)
    new_archive_store = ArchiveStore(data)
    new_metadata_store = MetadataStore(data)
    monkeypatch.setattr(storage_module, "comment_store", new_comment_store)
    monkeypatch.setattr(storage_module, "archive_store", new_archive_store)
    monkeypatch.setattr(storage_module, "metadata_store", new_metadata_store)
    # Also patch the imported names in routers
    import backend.routers.items as r_items
    import backend.routers.comments as r_comments
    import backend.routers.archive as r_archive
    import backend.routers.export as r_export
    monkeypatch.setattr(r_items, "comment_store", new_comment_store)
    monkeypatch.setattr(r_items, "metadata_store", new_metadata_store)
    monkeypatch.setattr(r_comments, "comment_store", new_comment_store)
    monkeypatch.setattr(r_archive, "comment_store", new_comment_store)
    monkeypatch.setattr(r_archive, "archive_store", new_archive_store)
    monkeypatch.setattr(r_export, "comment_store", new_comment_store)
    from backend.main import app
    with TestClient(app) as c:
        yield c


@pytest.fixture
def plan_id():
    return "plans/sprint-1-design"


@pytest.fixture
def design_id():
    return "designs/ui-mockup-v1"


@pytest.fixture
def proto_id():
    return "prototypes/review-flow"
