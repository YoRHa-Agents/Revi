use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ---------------------------------------------------------------------------
// Shared sub-types (field names are single-word so case doesn't matter)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename = "type")]
    pub ref_type: String,
    #[serde(default)]
    pub value: Value,
    pub section: Option<String>,
    pub label: Option<String>,
}

// ---------------------------------------------------------------------------
// DISK types — snake_case (matches Python model_dump_json() output)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentDisk {
    pub id: String,
    pub item_id: String,
    pub author: String,
    pub content: String,
    pub reference: Option<Reference>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveBatchDisk {
    pub archived_at: DateTime<Utc>,
    pub comments: Vec<CommentDisk>,
}

// ---------------------------------------------------------------------------
// WIRE types — camelCase (what the HTTP API sends back to the frontend)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub item_id: String,
    pub author: String,
    pub content: String,
    pub reference: Option<Reference>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

impl From<CommentDisk> for Comment {
    fn from(d: CommentDisk) -> Self {
        Self {
            id: d.id,
            item_id: d.item_id,
            author: d.author,
            content: d.content,
            reference: d.reference,
            status: d.status,
            created_at: d.created_at,
            resolved_at: d.resolved_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveBatch {
    pub archived_at: DateTime<Utc>,
    pub comments: Vec<Comment>,
}

impl From<ArchiveBatchDisk> for ArchiveBatch {
    fn from(d: ArchiveBatchDisk) -> Self {
        Self {
            archived_at: d.archived_at,
            comments: d.comments.into_iter().map(Comment::from).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewItem {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub title: String,
    pub title_zh: Option<String>,
    pub description: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub open_count: i64,
    pub resolved_count: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewItemDetail {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub title: String,
    pub title_zh: Option<String>,
    pub description: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub open_count: i64,
    pub resolved_count: i64,
    pub content_url: Option<String>,
    pub content_text: Option<String>,
}

// Request bodies (accept camelCase from clients)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCommentRequest {
    pub author: String,
    pub content: String,
    pub reference: Option<Reference>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTypeRequest {
    #[serde(rename = "type")]
    pub item_type: String,
}

// Export types
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportSummary {
    pub total: i64,
    pub open: i64,
    pub resolved: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportItem {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub title: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportComment {
    pub id: String,
    pub author: String,
    pub content: String,
    pub reference: Option<Reference>,
    pub created_at: DateTime<Utc>,
}

impl From<CommentDisk> for ExportComment {
    fn from(d: CommentDisk) -> Self {
        Self {
            id: d.id,
            author: d.author,
            content: d.content,
            reference: d.reference,
            created_at: d.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResponse {
    pub schema_version: String,
    pub item: ExportItem,
    pub summary: ExportSummary,
    pub open_comments: Vec<ExportComment>,
    pub exported_at: DateTime<Utc>,
}

// Upload
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadResponse {
    pub item_id: String,
    pub filename: String,
    pub url: String,
}

// Config
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigResponse {
    pub workspace_path: Option<String>,
    pub workspace_configured: bool,
    pub data_path: String,
    pub port: u16,
    pub config_file: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConfigRequest {
    pub workspace_path: Option<String>,
    pub data_path: Option<String>,
    pub port: Option<u16>,
}
