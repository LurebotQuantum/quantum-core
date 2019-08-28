
mod result;
mod plugin;

use log::{debug};
use result::Result;
use plugin::Plugin;
use std::ffi::OsStr;
use std::path::Path;

/// The Quantum Core
#[derive(Debug)]
pub struct Quantum {
    plugin_dir: String,
    plugins: Vec<Plugin>,
}

impl Quantum {
    /// Creates a new Quantum instance
    pub fn new() -> Quantum {
        Quantum {
            plugin_dir: ".".to_owned(),
            plugins: Vec::new(),
        }
    }

    /// Loads a plugin from a shared library
    pub fn load_plugin<S: AsRef<OsStr> + ?Sized>(&mut self, path: &S)
        -> Result<String> {
        let mut path = Path::new(path).to_path_buf();
        if path.is_relative() {
            path = Path::new(&self.plugin_dir).join(path);
        }
        let plugin = Plugin::load(path)?;
        let name = plugin.name().to_owned();
        self.plugins.push(plugin);
        Ok(name)
    }
}
