use std::path::Path;

use crate::analyzer::Analyzer;
use crate::drive::DriveService;
use crate::model::analysis::Analysis;
use crate::model::drive::Drive;
use crate::model::scan_result::ScanResult;
use crate::scanner::{ScanProgress, Scanner};

pub struct Engine {
    scanner: Scanner,
    analyzer: Analyzer,
    drive_service: DriveService,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            scanner: Scanner::new(),
            analyzer: Analyzer::new(),
            drive_service: DriveService::new(),
        }
    }

    pub fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    pub fn scan<F>(&self, path: &Path, progress: &mut F) -> ScanResult
    where
        F: FnMut(ScanProgress),
    {
        self.scanner.scan(path, progress)
    }

    pub fn analyze(&self, result: &ScanResult) -> Analysis {
        self.analyzer.analyze(&result.entries)
    }

    pub fn drives(&self) -> Vec<Drive> {
        self.drive_service.get_drives()
    }
}
