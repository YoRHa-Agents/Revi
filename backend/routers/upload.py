import shutil
from pathlib import Path
from typing import Optional
from fastapi import APIRouter, UploadFile, File, Form, HTTPException, status
from ..models import UploadResponse
from ..config import settings

router = APIRouter()

TYPE_MAP = {
    ".md": ("plans", "plan"),
    ".html": ("prototypes", "prototype"),
    ".htm": ("prototypes", "prototype"),
    ".png": ("designs", "design"),
    ".jpg": ("designs", "design"),
    ".jpeg": ("designs", "design"),
    ".svg": ("designs", "design"),
    ".webp": ("designs", "design"),
    ".pdf": ("designs", "design"),
}

SUBFOLDER_MAP = {
    "plan": "plans",
    "design": "designs",
    "prototype": "prototypes",
}


@router.post("/upload", response_model=UploadResponse, status_code=status.HTTP_201_CREATED)
async def upload_file(
    file: UploadFile = File(...),
    type: Optional[str] = Form(None),
):
    if type is not None and type not in SUBFOLDER_MAP:
        raise HTTPException(status_code=400, detail="type must be plan, design, or prototype")
    suffix = Path(file.filename).suffix.lower()
    if type is not None:
        subfolder = SUBFOLDER_MAP[type]
    else:
        if suffix not in TYPE_MAP:
            raise HTTPException(status_code=400, detail=f"Unsupported file type: {suffix}")
        subfolder, _ = TYPE_MAP[suffix]
    dest_dir = settings.workspace_path / subfolder
    dest_dir.mkdir(parents=True, exist_ok=True)
    dest = dest_dir / file.filename
    with dest.open("wb") as f:
        shutil.copyfileobj(file.file, f)
    stem = Path(file.filename).stem
    item_id = f"{subfolder}/{stem}"
    url = f"/workspace/{subfolder}/{file.filename}"
    return UploadResponse(item_id=item_id, filename=file.filename, url=url)
