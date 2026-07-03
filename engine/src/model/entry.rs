use std::path::PathBuf;

use crate::model::entry_attributes::EntryAttributes;
use crate::model::entry_metadata::EntryMetadata;

#[derive(Debug, Clone, Copy)]
pub enum EntryType {
    File,
    Directory,
}

pub struct Entry {
    // Identity
    pub name: String,
    pub path: PathBuf,
    pub entry_type: EntryType,

    // Metadata
    pub metadata: EntryMetadata,

    // Attributes
    pub attributes: EntryAttributes,

    // Relations
    pub children: Vec<Entry>,
}
