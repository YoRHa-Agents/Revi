from __future__ import annotations

import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Optional

from .models import ReviewItem, ReviewItemDetail

DESIGN_EXTS  = {".png", ".jpg", ".jpeg", ".svg", ".webp", ".pdf"}
PLAN_EXTS    = {".md"}
PROTO_EXTS   = {".html"}


def _stem_to_title(stem: str) -> str:
    return stem.replace("-", " ").replace("_", " ").title()


def _mtime(p: Path) -> datetime:
    return datetime.fromtimestamp(p.stat().st_mtime, tz=timezone.utc)


class WorkspaceScanner:
    def __init__(self, workspace_path: Path):
        self._root = workspace_path

    def _item_id(self, subfolder: str, stem: str) -> str:
        return f"{subfolder}/{stem}"

    def scan(self, overrides: dict | None = None) -> list[ReviewItem]:
        items = []
        for subfolder, exts, itype in [
            ("plans", PLAN_EXTS, "plan"),
            ("designs", DESIGN_EXTS, "design"),
            ("prototypes", PROTO_EXTS, "prototype"),
        ]:
            folder = self._root / subfolder
            if not folder.exists():
                continue
            for p in sorted(folder.iterdir()):
                if p.suffix.lower() not in exts:
                    continue
                item_id = self._item_id(subfolder, p.stem)
                title = self._extract_title(p, itype) or _stem_to_title(p.stem)
                items.append(ReviewItem(
                    id=item_id,
                    type=itype,
                    title=title,
                    updated_at=_mtime(p),
                    open_count=0,
                    resolved_count=0,
                ))
        items.sort(key=lambda x: x.updated_at, reverse=True)
        if overrides:
            for item in items:
                if item.id in overrides:
                    ov = overrides[item.id]
                    if "type" in ov:
                        item.type = ov["type"]
        return items

    def get_item(self, item_id: str, overrides: dict | None = None) -> Optional[ReviewItem]:
        for item in self.scan(overrides=overrides):
            if item.id == item_id:
                return item
        return None

    def get_detail(self, item_id: str, content_url_prefix: str = "/workspace", overrides: dict | None = None) -> Optional[ReviewItemDetail]:
        item = self.get_item(item_id, overrides=overrides)
        if not item:
            return None
        parts = item_id.split("/", 1)
        if len(parts) != 2:
            return None
        subfolder, stem = parts
        # find file
        folder = self._root / subfolder
        file_path = None
        for ext in (PLAN_EXTS | DESIGN_EXTS | PROTO_EXTS):
            candidate = folder / f"{stem}{ext}"
            if candidate.exists():
                file_path = candidate
                break
        content_url = f"{content_url_prefix}/{subfolder}/{stem}{file_path.suffix}" if file_path else None
        content_text = None
        if item.type == "plan" and file_path and file_path.exists():
            content_text = file_path.read_text(encoding="utf-8")
        detail_data = item.model_dump()
        detail_data["content_url"] = content_url
        detail_data["content_text"] = content_text
        return ReviewItemDetail(**detail_data)

    def _extract_title(self, path: Path, itype: str) -> Optional[str]:
        if itype != "plan":
            return None
        try:
            for line in path.read_text(encoding="utf-8").splitlines():
                m = re.match(r"^#\s+(.+)", line)
                if m:
                    return m.group(1).strip()
        except Exception:
            pass
        return None
