from fastapi import APIRouter, HTTPException, status
from ..models import ArchiveBatch
from ..storage import comment_store, archive_store

router = APIRouter()

@router.post("/archive/{item_id:path}", response_model=ArchiveBatch, status_code=status.HTTP_200_OK)
def archive_resolved(item_id: str):
    resolved = comment_store.remove_resolved(item_id)
    if not resolved:
        raise HTTPException(status_code=400, detail="No resolved comments to archive")
    return archive_store.add_batch(item_id, resolved)

@router.get("/archive/{item_id:path}", response_model=list[ArchiveBatch])
def list_archive(item_id: str):
    return archive_store.list(item_id)
