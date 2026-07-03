use std::time::SystemTime;

pub struct EntryMetadata {
    pub logical_size: u64,
    pub created: Option<SystemTime>,
    pub modified: Option<SystemTime>,
    pub accessed: Option<SystemTime>,
}
