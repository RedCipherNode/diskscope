use std::{path::Path, thread};

use engine::Engine;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Serialize)]
struct DriveDto {
    name: String,
    path: String,
}

#[derive(Serialize, Clone)]
struct ScanSummaryDto {
    total_files: u64,
    total_directories: u64,
    total_size: u64,
}

#[tauri::command]
fn get_engine_version() -> String {
    Engine::new().version().to_string()
}

#[tauri::command]
fn get_drives() -> Vec<DriveDto> {
    Engine::new()
        .drives()
        .into_iter()
        .map(|drive| DriveDto {
            name: drive.name,
            path: drive.path,
        })
        .collect()
}

#[tauri::command]
fn start_scan(app: AppHandle, path: String) {
    thread::spawn(move || {
        let engine = Engine::new();

        let mut progress = |_| {};

        let result = engine.scan(Path::new(&path), &mut progress);

        let analysis = engine.analyze(&result);

        let summary = ScanSummaryDto {
            total_files: analysis.total_files,
            total_directories: analysis.total_directories,
            total_size: analysis.total_logical_size,
        };

        app.emit("scan-finished", summary)
            .expect("failed to emit scan-finished");
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_engine_version,
            get_drives,
            start_scan
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
