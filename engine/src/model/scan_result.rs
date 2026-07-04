use crate::model::entry::Entry;
use crate::model::scan_error::ScanError;

pub struct ScanResult {
    pub entries: Vec<Entry>,
    pub errors: Vec<ScanError>,
}
