use std::fs::Metadata;
use std::path::PathBuf;
use std::time::SystemTime;
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub metadata: Metadata,
}

impl FileEntry {
    pub fn new(path: PathBuf, metadata: Metadata) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        Self { name, path, metadata }
    }

    pub fn is_dir(&self) -> bool {
        self.metadata.is_dir()
    }

    pub fn is_hidden(&self) -> bool {
        self.name.starts_with('.')
    }

    pub fn modified_time(&self) -> SystemTime {
        self.metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH)
    }
}
