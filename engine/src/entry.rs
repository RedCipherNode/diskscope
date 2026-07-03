use std::path::PathBuf;

pub enum EntryType {
    File,
    Directory,
}

pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub entry_type: EntryType,
}
