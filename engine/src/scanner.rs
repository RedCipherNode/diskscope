use std::fs;
use std::path::Path;

use crate::entry::{Entry, EntryType};
use crate::scan_result::ScanResult;

pub struct Scanner;

impl Scanner {
    pub fn new() -> Self {
        Self
    }

    pub fn scan(&self, path: &Path) -> ScanResult {
        let mut entries = Vec::new();

        if let Ok(directory) = fs::read_dir(path) {
            for item in directory {
                if let Ok(item) = item {
                    let entry_type = if item.path().is_dir() {
                        EntryType::Directory
                    } else {
                        EntryType::File
                    };

                    entries.push(Entry {
                        name: item.file_name().to_string_lossy().into_owned(),
                        path: item.path(),
                        entry_type,
                    });
                }
            }
        }

        ScanResult { entries }
    }
}
