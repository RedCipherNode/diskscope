use std::path::PathBuf;

pub struct LargestFile {
    pub path: PathBuf,
    pub logical_size: u64,
}
