from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.staticfiles import StaticFiles

from .config import settings
from .routers import items, comments, archive, export, upload

app = FastAPI(title="Revi API", version="1.0")

app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.allowed_origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Serve workspace files (designs/prototypes)
settings.workspace_path.mkdir(parents=True, exist_ok=True)
app.mount("/workspace", StaticFiles(directory=str(settings.workspace_path)), name="workspace")

app.include_router(items.router,    prefix="/api")
app.include_router(comments.router, prefix="/api")
app.include_router(archive.router,  prefix="/api")
app.include_router(export.router,   prefix="/api")
app.include_router(upload.router,   prefix="/api")
