from fastapi import APIRouter, HTTPException
from ..models import ReviewItem, ReviewItemDetail, UpdateTypeRequest
from ..config import settings
from ..workspace import WorkspaceScanner
from ..storage import comment_store, metadata_store

router = APIRouter()


def _scanner():
    return WorkspaceScanner(settings.workspace_path)


@router.get("/reviews", response_model=list[ReviewItem])
def list_reviews():
    scanner = _scanner()
    overrides = metadata_store.get_all()
    items = scanner.scan(overrides=overrides)
    for item in items:
        item.open_count = comment_store.open_count(item.id)
        item.resolved_count = comment_store.resolved_count(item.id)
    return items


@router.patch("/reviews/{item_id:path}", response_model=ReviewItem)
def update_review_type(item_id: str, body: UpdateTypeRequest):
    if body.type not in {"plan", "design", "prototype"}:
        raise HTTPException(status_code=400, detail="type must be plan, design, or prototype")
    scanner = _scanner()
    overrides = metadata_store.get_all()
    item = scanner.get_item(item_id, overrides=overrides)
    if not item:
        raise HTTPException(status_code=404, detail="Item not found")
    metadata_store.set_type(item_id, body.type)
    updated_overrides = metadata_store.get_all()
    item = scanner.get_item(item_id, overrides=updated_overrides)
    item.open_count = comment_store.open_count(item_id)
    item.resolved_count = comment_store.resolved_count(item_id)
    return item


@router.get("/reviews/{item_id:path}", response_model=ReviewItemDetail)
def get_review(item_id: str):
    scanner = _scanner()
    overrides = metadata_store.get_all()
    detail = scanner.get_detail(item_id, overrides=overrides)
    if not detail:
        raise HTTPException(status_code=404, detail="Item not found")
    detail.open_count = comment_store.open_count(item_id)
    detail.resolved_count = comment_store.resolved_count(item_id)
    return detail
