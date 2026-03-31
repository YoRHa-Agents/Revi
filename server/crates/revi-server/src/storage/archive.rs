use std::{collections::HashMap, path::PathBuf, sync::Mutex};

use anyhow::Context;
use chrono::Utc;

use crate::models::{ArchiveBatch, ArchiveBatchDisk, CommentDisk};

pub struct ArchiveStore {
    dir: PathBuf,
    cache: Mutex<HashMap<String, Vec<ArchiveBatchDisk>>>,
}

impl ArchiveStore {
    pub fn new(data_path: &std::path::Path) -> anyhow::Result<Self> {
        let dir = data_path.join("archive");
        std::fs::create_dir_all(&dir).context("create archive dir")?;
        Ok(Self { dir, cache: Mutex::new(HashMap::new()) })
    }

    fn safe_key(item_id: &str) -> String {
        item_id.replace('/', "__")
    }

    fn file_path(&self, item_id: &str) -> PathBuf {
        self.dir.join(format!("{}.json", Self::safe_key(item_id)))
    }

    fn load(&self, item_id: &str) -> anyhow::Result<Vec<ArchiveBatchDisk>> {
        let mut cache = self.cache.lock().unwrap();
        if let Some(v) = cache.get(item_id) {
            return Ok(v.clone());
        }
        let path = self.file_path(item_id);
        let data: Vec<ArchiveBatchDisk> = if path.exists() {
            let text = std::fs::read_to_string(&path).context("read archive file")?;
            serde_json::from_str(&text).context("parse archive")?
        } else {
            vec![]
        };
        cache.insert(item_id.to_owned(), data.clone());
        Ok(data)
    }

    fn save(&self, item_id: &str, data: Vec<ArchiveBatchDisk>) -> anyhow::Result<()> {
        let path = self.file_path(item_id);
        let tmp = path.with_extension("tmp");
        std::fs::write(&tmp, serde_json::to_string(&data)?).context("write tmp")?;
        std::fs::rename(&tmp, &path).context("rename")?;
        self.cache.lock().unwrap().insert(item_id.to_owned(), data);
        Ok(())
    }

    pub fn add_batch(
        &self,
        item_id: &str,
        comments: Vec<CommentDisk>,
    ) -> anyhow::Result<ArchiveBatch> {
        let mut data = self.load(item_id)?;
        let batch = ArchiveBatchDisk { archived_at: Utc::now(), comments };
        data.insert(0, batch.clone());
        self.save(item_id, data)?;
        Ok(ArchiveBatch::from(batch))
    }

    pub fn list(&self, item_id: &str) -> anyhow::Result<Vec<ArchiveBatch>> {
        Ok(self.load(item_id)?.into_iter().map(ArchiveBatch::from).collect())
    }
}
