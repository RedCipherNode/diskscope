use std::path::Path;

use crate::analyzer::Analyzer;
use crate::model::analysis::Analysis;
use crate::model::scan_result::ScanResult;
use crate::scanner::Scanner;

pub struct Engine {
    scanner: Scanner,
    analyzer: Analyzer,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            scanner: Scanner::new(),
            analyzer: Analyzer::new(),
        }
    }

    pub fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    pub fn scan(&self, path: &Path) -> ScanResult {
        self.scanner.scan(path)
    }

    pub fn analyze(&self, result: &ScanResult) -> Analysis {
        self.analyzer.analyze(&result.entries)
    }
}
