from fastapi import APIRouter, HTTPException, status
from ..models import Comment, AddCommentRequest
from ..config import settings
from ..workspace import WorkspaceScanner
from ..storage import comment_store

router = APIRouter()

def _item_exists(item_id: str) -> bool:
    return WorkspaceScanner(settings.workspace_path).get_item(item_id) is not None

# Define PATCH first (more specific), then GET/POST (catch-all)
@router.patch("/comments/{item_id:path}/{comment_id}/resolve", response_model=Comment)
def resolve_comment(item_id: str, comment_id: str):
    comment = comment_store.resolve(item_id, comment_id)
    if not comment:
        raise HTTPException(status_code=404, detail="Comment not found")
    return comment

@router.get("/comments/{item_id:path}", response_model=list[Comment])
def list_comments(item_id: str):
    return comment_store.list(item_id)

@router.post("/comments/{item_id:path}", response_model=Comment, status_code=status.HTTP_201_CREATED)
def add_comment(item_id: str, body: AddCommentRequest):
    if not _item_exists(item_id):
        raise HTTPException(status_code=404, detail="Item not found")
    return comment_store.add(item_id, body.author, body.content, body.reference)
