use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

use chrono::{DateTime, Utc};

use crate::models::{ReviewItem, ReviewItemDetail};

const DESIGN_EXTS: &[&str] = &["png", "jpg", "jpeg", "svg", "webp", "pdf"];
const PLAN_EXTS: &[&str] = &["md"];
const PROTO_EXTS: &[&str] = &["html", "htm"];

pub struct WorkspaceScanner {
    workspace: PathBuf,
}

impl WorkspaceScanner {
    pub fn new(workspace: PathBuf) -> Self {
        Self { workspace }
    }

    fn item_id(subfolder: &str, stem: &str) -> String {
        format!("{}/{}", subfolder, stem)
    }

    fn mtime(path: &Path) -> DateTime<Utc> {
        path.metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| {
                DateTime::from_timestamp(d.as_secs() as i64, d.subsec_nanos())
                    .unwrap_or_else(Utc::now)
            })
            .unwrap_or_else(Utc::now)
    }

    fn stem_to_title(stem: &str) -> String {
        stem.split(['-', '_'])
            .filter(|w| !w.is_empty())
            .map(|w| {
                let mut chars = w.chars();
                match chars.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn extract_title(path: &Path, item_type: &str) -> String {
        if item_type == "plan" {
            if let Ok(text) = std::fs::read_to_string(path) {
                for line in text.lines() {
                    if let Some(rest) = line.strip_prefix("# ") {
                        let title = rest.trim().to_owned();
                        if !title.is_empty() {
                            return title;
                        }
                    }
                }
            }
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        Self::stem_to_title(stem)
    }

    fn determine_type(ext: &str) -> Option<&'static str> {
        let ext = ext.to_lowercase();
        if PLAN_EXTS.contains(&ext.as_str()) {
            Some("plan")
        } else if DESIGN_EXTS.contains(&ext.as_str()) {
            Some("design")
        } else if PROTO_EXTS.contains(&ext.as_str()) {
            Some("prototype")
        } else {
            None
        }
    }

    pub fn scan(
        &self,
        overrides: &HashMap<String, serde_json::Value>,
    ) -> Vec<ReviewItem> {
        let subfolder_configs = [
            ("plans", "plan"),
            ("designs", "design"),
            ("prototypes", "prototype"),
        ];

        let mut items: Vec<(DateTime<Utc>, ReviewItem)> = Vec::new();

        for (subfolder, default_type) in &subfolder_configs {
            let dir = self.workspace.join(subfolder);
            if !dir.is_dir() {
                continue;
            }
            let read_dir = match std::fs::read_dir(&dir) {
                Ok(rd) => rd,
                Err(_) => continue,
            };
            for entry in read_dir.flatten() {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                if Self::determine_type(&ext).is_none() {
                    continue;
                }
                let stem = match path.file_stem().and_then(|s| s.to_str()) {
                    Some(s) => s.to_owned(),
                    None => continue,
                };
                let id = Self::item_id(subfolder, &stem);

                // Apply type override
                let item_type = overrides
                    .get(&id)
                    .and_then(|v| v.get("type"))
                    .and_then(|t| t.as_str())
                    .unwrap_or(default_type)
                    .to_owned();

                let title = Self::extract_title(&path, &item_type);
                let updated_at = Self::mtime(&path);

                items.push((
                    updated_at,
                    ReviewItem {
                        id,
                        item_type,
                        title,
                        title_zh: None,
                        description: None,
                        updated_at,
                        open_count: 0,
                        resolved_count: 0,
                    },
                ));
            }
        }

        // Sort newest first
        items.sort_by(|a, b| b.0.cmp(&a.0));
        items.into_iter().map(|(_, item)| item).collect()
    }

    pub fn get_item(
        &self,
        item_id: &str,
        overrides: &HashMap<String, serde_json::Value>,
    ) -> Option<ReviewItem> {
        self.scan(overrides).into_iter().find(|i| i.id == item_id)
    }

    pub fn find_file(&self, item_id: &str) -> Option<PathBuf> {
        let parts: Vec<&str> = item_id.splitn(2, '/').collect();
        if parts.len() != 2 {
            return None;
        }
        let (subfolder, stem) = (parts[0], parts[1]);
        let dir = self.workspace.join(subfolder);
        let read_dir = std::fs::read_dir(&dir).ok()?;
        for entry in read_dir.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if path.file_stem().and_then(|s| s.to_str()) == Some(stem) {
                return Some(path);
            }
        }
        None
    }

    pub fn get_detail(
        &self,
        item_id: &str,
        content_url_prefix: &str,
        overrides: &HashMap<String, serde_json::Value>,
    ) -> Option<ReviewItemDetail> {
        let item = self.get_item(item_id, overrides)?;
        let file_path = self.find_file(item_id)?;

        let rel = file_path.strip_prefix(&self.workspace).ok()?;
        let content_url = Some(format!(
            "{}/{}",
            content_url_prefix.trim_end_matches('/'),
            rel.to_string_lossy()
        ));

        let content_text = if item.item_type == "plan" {
            std::fs::read_to_string(&file_path).ok()
        } else {
            None
        };

        Some(ReviewItemDetail {
            id: item.id,
            item_type: item.item_type,
            title: item.title,
            title_zh: item.title_zh,
            description: item.description,
            updated_at: item.updated_at,
            open_count: item.open_count,
            resolved_count: item.resolved_count,
            content_url,
            content_text,
        })
    }
}
