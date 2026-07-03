use std::fs;
use std::path::Path;

use crate::model::entry::{Entry, EntryType};
use crate::model::entry_attributes::EntryAttributes;
use crate::model::entry_metadata::EntryMetadata;
use crate::model::scan_result::ScanResult;

pub struct Scanner;

impl Scanner {
    pub fn new() -> Self {
        Self
    }

    pub fn scan(&self, path: &Path) -> ScanResult {
        ScanResult {
            entries: self.scan_directory(path),
        }
    }

    fn scan_directory(&self, path: &Path) -> Vec<Entry> {
        let mut entries = Vec::new();

        if let Ok(directory) = fs::read_dir(path) {
            for item in directory {
                if let Ok(item) = item {
                    let path = item.path();

                    let entry_type = if path.is_dir() {
                        EntryType::Directory
                    } else {
                        EntryType::File
                    };

                    let metadata = item.metadata().ok();

                    let logical_size = match &metadata {
                        Some(metadata) if metadata.is_file() => metadata.len(),
                        _ => 0,
                    };

                    let children = match entry_type {
                        EntryType::Directory => self.scan_directory(&path),
                        EntryType::File => Vec::new(),
                    };

                    entries.push(Entry {
                        name: item.file_name().to_string_lossy().into_owned(),
                        path: path.clone(),

                        entry_type,

                        metadata: EntryMetadata {
                            logical_size,
                            created: metadata
                                .as_ref()
                                .and_then(|metadata| metadata.created().ok()),
                            modified: metadata
                                .as_ref()
                                .and_then(|metadata| metadata.modified().ok()),
                            accessed: metadata
                                .as_ref()
                                .and_then(|metadata| metadata.accessed().ok()),
                        },

                        attributes: EntryAttributes {
                            hidden: false,
                            system: false,
                            read_only: false,
                            archive: false,
                        },

                        children,
                    });
                }
            }
        }

        entries
    }
}
