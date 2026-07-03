use crate::model::analysis::Analysis;
use crate::model::entry::{Entry, EntryType};

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
        }
    }

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
}
