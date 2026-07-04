use std::path::PathBuf;

pub struct ScanError {
    pub path: PathBuf,
    pub message: String,
}
