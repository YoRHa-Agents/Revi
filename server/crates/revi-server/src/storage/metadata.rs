use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;

pub struct MetadataStore {
    path: PathBuf,
}

impl MetadataStore {
    pub fn new(data_path: &std::path::Path) -> Self {
        Self { path: data_path.join("metadata.json") }
    }

    pub fn load(&self) -> anyhow::Result<HashMap<String, serde_json::Value>> {
        if !self.path.exists() {
            return Ok(HashMap::new());
        }
        let text =
            std::fs::read_to_string(&self.path).context("read metadata.json")?;
        serde_json::from_str(&text).context("parse metadata.json")
    }

    pub fn set_type(&self, item_id: &str, item_type: &str) -> anyhow::Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).context("create data dir")?;
        }
        let mut data = self.load().unwrap_or_default();
        let entry = data
            .entry(item_id.to_owned())
            .or_insert_with(|| serde_json::Value::Object(Default::default()));
        if let serde_json::Value::Object(map) = entry {
            map.insert("type".to_owned(), serde_json::Value::String(item_type.to_owned()));
        }
        let tmp = self.path.with_extension("tmp");
        let json = serde_json::to_string_pretty(&data).context("serialize metadata")?;
        std::fs::write(&tmp, json).context("write tmp")?;
        std::fs::rename(&tmp, &self.path).context("rename")?;
        Ok(())
    }
}
