//! Small, deterministic scripting boundary.
//!
//! This module deliberately does not embed a JavaScript engine.  The browser
//! can use [`Runtime`] to queue scripts for a backend, while tests and native
//! front-ends can inspect the exact order in which scripts were requested.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Script {
    pub source: String,
}

#[derive(Clone, Debug, Default)]
pub struct Runtime {
    scripts: Vec<Script>,
}

impl Runtime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enqueue(&mut self, source: impl Into<String>) {
        self.scripts.push(Script { source: source.into() });
    }

    pub fn pending(&self) -> &[Script] {
        &self.scripts
    }

    pub fn drain(&mut self) -> Vec<Script> {
        std::mem::take(&mut self.scripts)
    }

    pub fn is_empty(&self) -> bool {
        self.scripts.is_empty()
    }
}
