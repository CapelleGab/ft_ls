use std::fs;
use std::path::Path;

use crate::entry::FileEntry;

pub fn list_entries(path: &Path, show_hidden: bool) -> Vec<FileEntry> {
    let read_dir = match fs::read_dir(path) {
        Ok(rd) => rd,
        Err(e) => {
            eprintln!("ft_ls: cannot access '{}': {}", path.display(), e);
            return Vec::new();
        }
    };

    let mut entries: Vec<FileEntry> = read_dir
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let path = e.path();
            let metadata = fs::symlink_metadata(&path).ok()?;
            Some(FileEntry::new(path, metadata))
        })
        .filter(|e| show_hidden || !e.is_hidden())
        .collect();

    entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    entries
}

pub fn sort_entries(entries: &mut Vec<FileEntry>, by_time: bool, reverse: bool) {
    if by_time {
        entries.sort_by(|a, b| b.modified_time().cmp(&a.modified_time()));
    }
    if reverse {
        entries.reverse();
    }
}
