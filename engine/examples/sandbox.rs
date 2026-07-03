use std::path::Path;

use engine::engine::Engine;
use engine::model::entry::Entry;
use engine::model::entry::EntryType;

fn main() {
    let engine = Engine::new();

    let result = engine.scan(Path::new("D:\\"));

    let analysis = engine.analyze(&result);

    println!("DiskScope {}\n", engine.version());

    print_entries(&result.entries, 0);
    println!();
    println!("Total Files : {}", analysis.total_files);
    println!("Total Directories : {}", analysis.total_directories);
    println!("Total Logical Size : {} B", analysis.total_logical_size);
}

fn print_entries(entries: &[Entry], depth: usize) {
    for entry in entries {
        let indent = "  ".repeat(depth);

        let icon = match entry.entry_type {
            EntryType::Directory => "📁",
            EntryType::File => "📄",
        };

        println!(
            "{}{} {} ({} B)",
            indent, icon, entry.name, entry.metadata.logical_size
        );

        print_entries(&entry.children, depth + 1);
    }
}
