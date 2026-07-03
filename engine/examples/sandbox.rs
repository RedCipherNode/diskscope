use std::path::Path;

use engine::engine::Engine;

fn main() {
    let engine = Engine::new();

    let result = engine.scan(Path::new("D:\\"));

    println!("DiskScope {}", engine.version());

    for entry in result.entries {
        println!("{}", entry.name);
    }
}
