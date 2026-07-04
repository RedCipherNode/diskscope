use std::collections::HashMap;

use crate::model::analysis::Analysis;
use crate::model::empty_directory::EmptyDirectory;
use crate::model::empty_file::EmptyFile;
use crate::model::entry::{Entry, EntryType};
use crate::model::extension_statistic::ExtensionStatistic;
use crate::model::largest_directory::LargestDirectory;
use crate::model::largest_file::LargestFile;

pub struct Analyzer;

impl Analyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, entries: &[Entry]) -> Analysis {
        Analysis {
            total_files: self.count_files(entries),
            total_directories: self.count_directories(entries),
            total_logical_size: self.count_logical_size(entries),
            extension_statistics: self.analyze_extensions(entries),
            largest_directories: self.analyze_largest_directories(entries),
            largest_files: self.analyze_largest_files(entries),
            empty_files: self.analyze_empty_files(entries),
            empty_directories: self.analyze_empty_directories(entries),
        }
    }

    // ==========================================================
    // Basic Statistics
    // ==========================================================

    fn count_files(&self, entries: &[Entry]) -> u64 {
        let mut total = 0;

        for entry in entries {
            match entry.entry_type {
                EntryType::File => {
                    total += 1;
                }

                EntryType::Directory => {
                    total += self.count_files(&entry.children);
                }
            }
        }

        total
    }

    fn count_directories(&self, entries: &[Entry]) -> u64 {
        let mut total = 0;

        for entry in entries {
            match entry.entry_type {
                EntryType::Directory => {
                    total += 1;
                    total += self.count_directories(&entry.children);
                }

                EntryType::File => {}
            }
        }

        total
    }

    fn count_logical_size(&self, entries: &[Entry]) -> u64 {
        let mut total = 0;

        for entry in entries {
            match entry.entry_type {
                EntryType::File => {
                    total += entry.metadata.logical_size;
                }

                EntryType::Directory => {
                    total += self.count_logical_size(&entry.children);
                }
            }
        }

        total
    }

    // ==========================================================
    // Extension Statistics
    // ==========================================================

    fn analyze_extensions(&self, entries: &[Entry]) -> Vec<ExtensionStatistic> {
        let mut statistics = HashMap::new();

        self.collect_extensions(entries, &mut statistics);

        let mut result: Vec<ExtensionStatistic> = statistics.into_values().collect();

        result.sort_by(|a, b| b.logical_size.cmp(&a.logical_size));

        result
    }

    fn collect_extensions(
        &self,
        entries: &[Entry],
        statistics: &mut HashMap<String, ExtensionStatistic>,
    ) {
        for entry in entries {
            match entry.entry_type {
                EntryType::File => {
                    if let Some(extension) = &entry.extension {
                        if !statistics.contains_key(extension) {
                            statistics.insert(
                                extension.clone(),
                                ExtensionStatistic {
                                    extension: extension.clone(),
                                    file_count: 1,
                                    logical_size: entry.metadata.logical_size,
                                },
                            );
                        } else if let Some(statistic) = statistics.get_mut(extension) {
                            statistic.file_count += 1;
                            statistic.logical_size += entry.metadata.logical_size;
                        }
                    }
                }

                EntryType::Directory => {
                    self.collect_extensions(&entry.children, statistics);
                }
            }
        }
    }

    fn analyze_largest_directories(&self, entries: &[Entry]) -> Vec<LargestDirectory> {
        let mut directories = Vec::new();

        self.collect_largest_directories(entries, &mut directories);

        directories.sort_by(|a, b| b.logical_size.cmp(&a.logical_size));

        directories
    }

    fn collect_largest_directories(
        &self,
        entries: &[Entry],
        directories: &mut Vec<LargestDirectory>,
    ) {
        for entry in entries {
            if let EntryType::Directory = entry.entry_type {
                directories.push(LargestDirectory {
                    path: entry.path.clone(),
                    logical_size: self.calculate_directory_size(entry),
                });

                self.collect_largest_directories(&entry.children, directories);
            }
        }
    }

    fn analyze_largest_files(&self, entries: &[Entry]) -> Vec<LargestFile> {
        let mut files = Vec::new();

        self.collect_largest_files(entries, &mut files);

        files.sort_by(|a, b| b.logical_size.cmp(&a.logical_size));

        files
    }

    fn collect_largest_files(&self, entries: &[Entry], files: &mut Vec<LargestFile>) {
        for entry in entries {
            match entry.entry_type {
                EntryType::File => {
                    files.push(LargestFile {
                        path: entry.path.clone(),
                        logical_size: entry.metadata.logical_size,
                    });
                }

                EntryType::Directory => {
                    self.collect_largest_files(&entry.children, files);
                }
            }
        }
    }

    fn analyze_empty_files(&self, entries: &[Entry]) -> Vec<EmptyFile> {
        let mut files = Vec::new();

        self.collect_empty_files(entries, &mut files);

        files
    }

    fn collect_empty_files(&self, entries: &[Entry], files: &mut Vec<EmptyFile>) {
        for entry in entries {
            match entry.entry_type {
                EntryType::File => {
                    if entry.metadata.logical_size == 0 {
                        files.push(EmptyFile {
                            path: entry.path.clone(),
                        });
                    }
                }

                EntryType::Directory => {
                    self.collect_empty_files(&entry.children, files);
                }
            }
        }
    }

    fn analyze_empty_directories(&self, entries: &[Entry]) -> Vec<EmptyDirectory> {
        let mut directories = Vec::new();

        self.collect_empty_directories(entries, &mut directories);

        directories
    }

    fn collect_empty_directories(&self, entries: &[Entry], directories: &mut Vec<EmptyDirectory>) {
        for entry in entries {
            if let EntryType::Directory = entry.entry_type {
                if entry.children.is_empty() {
                    directories.push(EmptyDirectory {
                        path: entry.path.clone(),
                    });
                } else {
                    self.collect_empty_directories(&entry.children, directories);
                }
            }
        }
    }

    fn calculate_directory_size(&self, entry: &Entry) -> u64 {
        let mut total = 0;

        for child in &entry.children {
            match child.entry_type {
                EntryType::File => {
                    total += child.metadata.logical_size;
                }

                EntryType::Directory => {
                    total += self.calculate_directory_size(child);
                }
            }
        }

        total
    }
}
