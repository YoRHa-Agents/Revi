from __future__ import annotations
import json
import os
from datetime import datetime, timezone
from pathlib import Path
from typing import Optional
from uuid import uuid4

from .config import settings
from .models import Comment, ArchiveBatch, Reference


def _now() -> datetime:
    return datetime.now(timezone.utc)


def _safe_key(item_id: str) -> str:
    return item_id.replace("/", "__")


class CommentStore:
    def __init__(self, data_path: Optional[Path] = None):
        self._path = (data_path or settings.data_path) / "comments"
        self._path.mkdir(parents=True, exist_ok=True)
        self._cache: dict[str, list[dict]] = {}

    def _file(self, item_id: str) -> Path:
        return self._path / f"{_safe_key(item_id)}.json"

    def _load(self, item_id: str) -> list[dict]:
        if item_id in self._cache:
            return self._cache[item_id]
        f = self._file(item_id)
        if not f.exists():
            self._cache[item_id] = []
            return []
        data = json.loads(f.read_text())
        self._cache[item_id] = data
        return data

    def _save(self, item_id: str, data: list[dict]):
        self._cache[item_id] = data
        f = self._file(item_id)
        tmp = f.with_suffix(".tmp")
        tmp.write_text(json.dumps(data, default=str))
        os.replace(tmp, f)

    def list(self, item_id: str) -> list[Comment]:
        return [Comment(**c) for c in self._load(item_id)]

    def add(self, item_id: str, author: str, content: str, reference=None) -> Comment:
        data = self._load(item_id)
        comment = Comment(
            id=str(uuid4()),
            item_id=item_id,
            author=author,
            content=content,
            reference=reference,
            status="open",
            created_at=_now(),
            resolved_at=None,
        )
        data.append(json.loads(comment.model_dump_json()))
        self._save(item_id, data)
        return comment

    def resolve(self, item_id: str, comment_id: str) -> Optional[Comment]:
        data = self._load(item_id)
        for c in data:
            if c["id"] == comment_id:
                if c["status"] != "resolved":
                    c["status"] = "resolved"
                    c["resolved_at"] = _now().isoformat()
                    self._save(item_id, data)
                return Comment(**c)
        return None

    def remove_resolved(self, item_id: str) -> list[dict]:
        data = self._load(item_id)
        resolved = [c for c in data if c["status"] == "resolved"]
        if not resolved:
            return []
        self._save(item_id, [c for c in data if c["status"] == "open"])
        return resolved

    def open_count(self, item_id: str) -> int:
        return sum(1 for c in self._load(item_id) if c["status"] == "open")

    def resolved_count(self, item_id: str) -> int:
        return sum(1 for c in self._load(item_id) if c["status"] == "resolved")


class ArchiveStore:
    def __init__(self, data_path: Optional[Path] = None):
        self._path = (data_path or settings.data_path) / "archive"
        self._path.mkdir(parents=True, exist_ok=True)
        self._cache: dict[str, list[dict]] = {}

    def _file(self, item_id: str) -> Path:
        return self._path / f"{_safe_key(item_id)}.json"

    def _load(self, item_id: str) -> list[dict]:
        if item_id in self._cache:
            return self._cache[item_id]
        f = self._file(item_id)
        if not f.exists():
            self._cache[item_id] = []
            return []
        data = json.loads(f.read_text())
        self._cache[item_id] = data
        return data

    def _save(self, item_id: str, data: list[dict]):
        self._cache[item_id] = data
        f = self._file(item_id)
        tmp = f.with_suffix(".tmp")
        tmp.write_text(json.dumps(data, default=str))
        os.replace(tmp, f)

    def add_batch(self, item_id: str, comments: list[dict]) -> ArchiveBatch:
        data = self._load(item_id)
        batch = {"archived_at": _now().isoformat(), "comments": comments}
        data.insert(0, batch)
        self._save(item_id, data)
        return ArchiveBatch(**batch)

    def list(self, item_id: str) -> list[ArchiveBatch]:
        return [ArchiveBatch(**b) for b in self._load(item_id)]


comment_store = CommentStore()
archive_store = ArchiveStore()


class MetadataStore:
    def __init__(self, data_path: Path):
        self._path = data_path / "metadata.json"
        self._data_path = data_path

    def _load(self) -> dict:
        if not self._path.exists():
            return {}
        try:
            return json.loads(self._path.read_text(encoding="utf-8"))
        except Exception:
            return {}

    def get_all(self) -> dict:
        return self._load()

    def set_type(self, item_id: str, item_type: str) -> None:
        self._data_path.mkdir(parents=True, exist_ok=True)
        data = self._load()
        if item_id not in data:
            data[item_id] = {}
        data[item_id]["type"] = item_type
        tmp = self._path.with_suffix(".tmp")
        tmp.write_text(json.dumps(data, indent=2, ensure_ascii=False), encoding="utf-8")
        os.replace(tmp, self._path)


metadata_store = MetadataStore(settings.data_path)
