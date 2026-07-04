use std::fs;
use std::os::windows::fs::MetadataExt;
use std::path::Path;

use windows_sys::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_ARCHIVE, FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_SYSTEM,
};

use crate::model::entry::{Entry, EntryType};
use crate::model::entry_attributes::EntryAttributes;
use crate::model::entry_metadata::EntryMetadata;
use crate::model::scan_error::ScanError;
use crate::model::scan_result::ScanResult;

pub struct Scanner;

impl Scanner {
    pub fn new() -> Self {
        Self
    }

    pub fn scan(&self, path: &Path) -> ScanResult {
        let mut errors = Vec::new();

        ScanResult {
            entries: self.scan_directory(path, &mut errors),
            errors,
        }
    }

    fn scan_directory(&self, path: &Path, errors: &mut Vec<ScanError>) -> Vec<Entry> {
        let mut entries = Vec::new();

        match fs::read_dir(path) {
            Ok(directory) => {
                for item in directory {
                    if let Ok(item) = item {
                        let path = item.path();

                        let entry_type = if path.is_dir() {
                            EntryType::Directory
                        } else {
                            EntryType::File
                        };

                        let extension = path
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .map(|ext| ext.to_ascii_lowercase());

                        let metadata = item.metadata().ok();

                        let logical_size = match &metadata {
                            Some(metadata) if metadata.is_file() => metadata.len(),
                            _ => 0,
                        };

                        let file_attributes = metadata
                            .as_ref()
                            .map(|metadata| metadata.file_attributes())
                            .unwrap_or(0);

                        let children = match entry_type {
                            EntryType::Directory => self.scan_directory(&path, errors),
                            EntryType::File => Vec::new(),
                        };

                        entries.push(Entry {
                            name: item.file_name().to_string_lossy().into_owned(),
                            extension,
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
                                hidden: (file_attributes & FILE_ATTRIBUTE_HIDDEN) != 0,
                                system: (file_attributes & FILE_ATTRIBUTE_SYSTEM) != 0,
                                read_only: (file_attributes & FILE_ATTRIBUTE_READONLY) != 0,
                                archive: (file_attributes & FILE_ATTRIBUTE_ARCHIVE) != 0,
                            },

                            children,
                        });
                    }
                }
            }

            Err(error) => {
                errors.push(ScanError {
                    path: path.to_path_buf(),
                    message: error.to_string(),
                });
            }
        }
        entries
    }
}
