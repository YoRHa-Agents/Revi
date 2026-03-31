from datetime import datetime, timezone
from fastapi import APIRouter, HTTPException
from ..models import ExportResponse, ExportItem, ExportSummary, ExportComment
from ..config import settings
from ..workspace import WorkspaceScanner
from ..storage import comment_store

router = APIRouter()

@router.get("/export/{item_id:path}", response_model=ExportResponse)
def export_for_agent(item_id: str):
    scanner = WorkspaceScanner(settings.workspace_path)
    item = scanner.get_item(item_id)
    if not item:
        raise HTTPException(status_code=404, detail="Item not found")
    all_comments = comment_store.list(item_id)
    open_comments = [c for c in all_comments if c.status == "open"]
    return ExportResponse(
        schema_version="1.0",
        item=ExportItem(id=item.id, type=item.type, title=item.title),
        summary=ExportSummary(
            total=len(all_comments),
            open=len(open_comments),
            resolved=sum(1 for c in all_comments if c.status == "resolved"),
        ),
        open_comments=[ExportComment(
            id=c.id, author=c.author, content=c.content,
            reference=c.reference, created_at=c.created_at,
        ) for c in open_comments],
        exported_at=datetime.now(timezone.utc),
    )
