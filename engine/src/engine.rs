use std::path::Path;

use crate::model::scan_result::ScanResult;
use crate::scanner::Scanner;

pub struct Engine {
    scanner: Scanner,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            scanner: Scanner::new(),
        }
    }

    pub fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    pub fn scan(&self, path: &Path) -> ScanResult {
        self.scanner.scan(path)
    }
}
