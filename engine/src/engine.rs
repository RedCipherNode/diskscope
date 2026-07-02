pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Self
    }

    pub fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_returns_version() {
        let engine = Engine::new();

        assert_eq!(engine.version(), env!("CARGO_PKG_VERSION"));
    }
}
