use std::path::Path;

use crate::model::drive::Drive;

pub struct DriveService;

impl DriveService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_drives(&self) -> Vec<Drive> {
        let mut drives = Vec::new();

        for letter in b'A'..=b'Z' {
            let path = format!("{}:\\", letter as char);

            if Path::new(&path).exists() {
                drives.push(Drive {
                    name: path.clone(),
                    path,
                });
            }
        }

        drives
    }
}
