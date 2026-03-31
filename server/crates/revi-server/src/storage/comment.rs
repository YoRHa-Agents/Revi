use std::{
    collections::HashMap,
    path::PathBuf,
    sync::Mutex,
};

use anyhow::Context;
use chrono::Utc;
use uuid::Uuid;

use crate::models::{Comment, CommentDisk, Reference};

pub struct CommentStore {
    dir: PathBuf,
    cache: Mutex<HashMap<String, Vec<CommentDisk>>>,
}

impl CommentStore {
    pub fn new(data_path: &std::path::Path) -> anyhow::Result<Self> {
        let dir = data_path.join("comments");
        std::fs::create_dir_all(&dir).context("create comments dir")?;
        Ok(Self { dir, cache: Mutex::new(HashMap::new()) })
    }

    fn safe_key(item_id: &str) -> String {
        item_id.replace('/', "__")
    }

    fn file_path(&self, item_id: &str) -> PathBuf {
        self.dir.join(format!("{}.json", Self::safe_key(item_id)))
    }

    fn load(&self, item_id: &str) -> anyhow::Result<Vec<CommentDisk>> {
        let mut cache = self.cache.lock().unwrap();
        if let Some(v) = cache.get(item_id) {
            return Ok(v.clone());
        }
        let path = self.file_path(item_id);
        let data: Vec<CommentDisk> = if path.exists() {
            let text = std::fs::read_to_string(&path).context("read comments file")?;
            serde_json::from_str(&text).context("parse comments")?
        } else {
            vec![]
        };
        cache.insert(item_id.to_owned(), data.clone());
        Ok(data)
    }

    fn save(&self, item_id: &str, data: Vec<CommentDisk>) -> anyhow::Result<()> {
        let path = self.file_path(item_id);
        let tmp = path.with_extension("tmp");
        std::fs::write(&tmp, serde_json::to_string(&data)?).context("write tmp")?;
        std::fs::rename(&tmp, &path).context("rename")?;
        self.cache.lock().unwrap().insert(item_id.to_owned(), data);
        Ok(())
    }

    pub fn list(&self, item_id: &str) -> anyhow::Result<Vec<Comment>> {
        Ok(self.load(item_id)?.into_iter().map(Comment::from).collect())
    }

    pub fn list_disk(&self, item_id: &str) -> anyhow::Result<Vec<CommentDisk>> {
        self.load(item_id)
    }

    pub fn add(
        &self,
        item_id: &str,
        author: String,
        content: String,
        reference: Option<Reference>,
    ) -> anyhow::Result<Comment> {
        let mut data = self.load(item_id)?;
        let disk = CommentDisk {
            id: Uuid::new_v4().to_string(),
            item_id: item_id.to_owned(),
            author,
            content,
            reference,
            status: "open".to_owned(),
            created_at: Utc::now(),
            resolved_at: None,
        };
        data.push(disk.clone());
        self.save(item_id, data)?;
        Ok(Comment::from(disk))
    }

    pub fn resolve(&self, item_id: &str, comment_id: &str) -> anyhow::Result<Option<Comment>> {
        let mut data = self.load(item_id)?;
        let mut found = false;
        for c in &mut data {
            if c.id == comment_id {
                found = true;
                if c.status != "resolved" {
                    c.status = "resolved".to_owned();
                    c.resolved_at = Some(Utc::now());
                }
                break;
            }
        }
        if !found {
            return Ok(None);
        }
        self.save(item_id, data.clone())?;
        Ok(data
            .into_iter()
            .find(|c| c.id == comment_id)
            .map(Comment::from))
    }

    /// Remove all resolved comments, return them as disk structs.
    pub fn remove_resolved(&self, item_id: &str) -> anyhow::Result<Vec<CommentDisk>> {
        let data = self.load(item_id)?;
        let resolved: Vec<CommentDisk> =
            data.iter().filter(|c| c.status == "resolved").cloned().collect();
        if resolved.is_empty() {
            return Ok(vec![]);
        }
        let open: Vec<CommentDisk> =
            data.into_iter().filter(|c| c.status == "open").collect();
        self.save(item_id, open)?;
        Ok(resolved)
    }

    pub fn open_count(&self, item_id: &str) -> anyhow::Result<i64> {
        Ok(self
            .load(item_id)?
            .iter()
            .filter(|c| c.status == "open")
            .count() as i64)
    }

    pub fn resolved_count(&self, item_id: &str) -> anyhow::Result<i64> {
        Ok(self
            .load(item_id)?
            .iter()
            .filter(|c| c.status == "resolved")
            .count() as i64)
    }
}
