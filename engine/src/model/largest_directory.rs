use std::path::PathBuf;

pub struct LargestDirectory {
    pub path: PathBuf,
    pub logical_size: u64,
}
