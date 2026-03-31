from __future__ import annotations
from datetime import datetime
from typing import Optional, Any
from uuid import UUID
from pydantic import BaseModel, ConfigDict, Field, field_validator
from pydantic.alias_generators import to_camel


class CamelModel(BaseModel):
    model_config = ConfigDict(alias_generator=to_camel, populate_by_name=True)


class Reference(CamelModel):
    type: str   # quote | annotation | step | section | general | line
    value: Any = None
    section: Optional[str] = None
    label: Optional[str] = None


class Comment(CamelModel):
    id: str
    item_id: str
    author: str
    content: str
    reference: Optional[Reference] = None
    status: str = "open"
    created_at: datetime
    resolved_at: Optional[datetime] = None


class ArchiveBatch(CamelModel):
    archived_at: datetime
    comments: list[Comment]


class ReviewItem(CamelModel):
    id: str
    type: str   # plan | design | prototype
    title: str
    title_zh: Optional[str] = None
    description: Optional[str] = None
    updated_at: datetime
    open_count: int = 0
    resolved_count: int = 0


class ReviewItemDetail(ReviewItem):
    content_url: Optional[str] = None
    content_text: Optional[str] = None


class AddCommentRequest(CamelModel):
    author: str
    content: str
    reference: Optional[Reference] = None


class ExportSummary(CamelModel):
    total: int
    open: int
    resolved: int


class ExportItem(CamelModel):
    id: str
    type: str
    title: str


class ExportComment(CamelModel):
    id: str
    author: str
    content: str
    reference: Optional[Reference] = None
    created_at: datetime


class ExportResponse(CamelModel):
    schema_version: str = "1.0"
    item: ExportItem
    summary: ExportSummary
    open_comments: list[ExportComment]
    exported_at: datetime


class UploadResponse(CamelModel):
    item_id: str
    filename: str
    url: str


class UpdateTypeRequest(CamelModel):
    type: str
