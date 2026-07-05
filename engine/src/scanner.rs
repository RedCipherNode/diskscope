use std::fs;
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};

use windows_sys::Win32::Storage::FileSystem::{
    FILE_ATTRIBUTE_ARCHIVE, FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_SYSTEM,
};

use crate::model::entry::{Entry, EntryType};
use crate::model::entry_attributes::EntryAttributes;
use crate::model::entry_metadata::EntryMetadata;
use crate::model::scan_error::ScanError;
use crate::model::scan_result::ScanResult;

pub struct ScanProgress {
    pub current_path: PathBuf,
    pub scanned_files: u64,
    pub scanned_directories: u64,
}

pub struct Scanner;

impl Scanner {
    pub fn new() -> Self {
        Self
    }

    pub fn scan<F>(&self, path: &Path, progress: &mut F) -> ScanResult
    where
        F: FnMut(ScanProgress),
    {
        let mut errors = Vec::new();

        let mut scanned_files = 0;
        let mut scanned_directories = 0;

        let entries = self.scan_directory(
            path,
            &mut errors,
            &mut scanned_files,
            &mut scanned_directories,
            progress,
        );

        ScanResult { entries, errors }
    }

    fn scan_directory<F>(
        &self,
        path: &Path,
        errors: &mut Vec<ScanError>,
        scanned_files: &mut u64,
        scanned_directories: &mut u64,
        progress: &mut F,
    ) -> Vec<Entry>
    where
        F: FnMut(ScanProgress),
    {
        let mut entries = Vec::new();

        match fs::read_dir(path) {
            Ok(directory) => {
                for item in directory {
                    if let Ok(item) = item {
                        let path = item.path();

                        let entry_type = if path.is_dir() {
                            *scanned_directories += 1;
                            EntryType::Directory
                        } else {
                            *scanned_files += 1;
                            EntryType::File
                        };

                        progress(ScanProgress {
                            current_path: path.clone(),
                            scanned_files: *scanned_files,
                            scanned_directories: *scanned_directories,
                        });

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
                            EntryType::Directory => self.scan_directory(
                                &path,
                                errors,
                                scanned_files,
                                scanned_directories,
                                progress,
                            ),
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
