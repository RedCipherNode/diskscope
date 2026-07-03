use std::path::Path;

use engine::engine::Engine;
use engine::model::entry::EntryType;

fn main() {
    let engine = Engine::new();

    let result = engine.scan(Path::new("D:\\"));

    println!("DiskScope {}\n", engine.version());

    for entry in result.entries {
        let icon = match entry.entry_type {
            EntryType::Directory => "📁",
            EntryType::File => "📄",
        };

        println!("{} {} ({} B)", icon, entry.name, entry.logical_size);
    }
}
