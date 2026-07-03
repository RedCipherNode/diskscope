use std::fs;
use std::path::Path;

use crate::model::entry::{Entry, EntryType};
use crate::model::entry_attributes::EntryAttributes;
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

                    let logical_size = match item.metadata() {
                        Ok(metadata) => {
                            if metadata.is_file() {
                                metadata.len()
                            } else {
                                0
                            }
                        }
                        Err(_) => 0,
                    };

                    let children = match entry_type {
                        EntryType::Directory => self.scan_directory(&path),
                        EntryType::File => Vec::new(),
                    };

                    entries.push(Entry {
                        name: item.file_name().to_string_lossy().into_owned(),
                        path: path.clone(),
                        entry_type,
                        logical_size,
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
