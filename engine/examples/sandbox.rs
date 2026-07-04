use std::path::Path;

use engine::engine::Engine;
use engine::formatter::size_formatter::SizeFormatter;
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
    println!(
        "Total Logical Size : {}",
        SizeFormatter::format(analysis.total_logical_size)
    );
    println!();
    println!("Extension Statistics:");

    for statistic in &analysis.extension_statistics {
        println!(
            ".{} : {} files ({})",
            statistic.extension,
            statistic.file_count,
            SizeFormatter::format(statistic.logical_size)
        );
    }
    println!();
    println!("Largest Directories:");
    for directory in analysis.largest_directories.iter().take(10) {
        println!(
            "{} ({})",
            directory.path.display(),
            SizeFormatter::format(directory.logical_size),
        );
    }
    println!();
    println!("Largest Files:");

    for file in analysis.largest_files.iter().take(10) {
        println!(
            "{} ({})",
            file.path.display(),
            SizeFormatter::format(file.logical_size),
        );
    }
    println!();
    println!("Empty Files:");

    for file in analysis.empty_files.iter().take(10) {
        println!("{}", file.path.display());
    }
    println!();
    println!("Empty Directories:");

    for directory in analysis.empty_directories.iter().take(10) {
        println!("{}", directory.path.display());
    }

    println!();
    println!("Scan Errors : {}", result.errors.len());

    for error in result.errors.iter().take(10) {
        println!("{} -> {}", error.path.display(), error.message);
    }
}

fn print_entries(entries: &[Entry], depth: usize) {
    for entry in entries {
        let indent = "  ".repeat(depth);

        let icon = match entry.entry_type {
            EntryType::Directory => "📁",
            EntryType::File => "📄",
        };

        println!(
            "{}{} {} ({}) [H:{} S:{} R:{} A:{}]",
            indent,
            icon,
            entry.name,
            SizeFormatter::format(entry.metadata.logical_size),
            entry.attributes.hidden,
            entry.attributes.system,
            entry.attributes.read_only,
            entry.attributes.archive,
        );

        print_entries(&entry.children, depth + 1);
    }
}
