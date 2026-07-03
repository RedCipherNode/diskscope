use std::path::PathBuf;

use crate::model::entry_attributes::EntryAttributes;

#[derive(Debug, Clone, Copy)]
pub enum EntryType {
    File,
    Directory,
}

pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub entry_type: EntryType,
    pub logical_size: u64,
    pub attributes: EntryAttributes,

    pub children: Vec<Entry>,
}
