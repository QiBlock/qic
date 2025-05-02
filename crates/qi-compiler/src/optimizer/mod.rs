//! The LLVM optimizing tools.

pub mod settings;

use serde::Deserialize;
use serde::Serialize;

use self::settings::Settings;

/// The LLVM optimizing tools.
#[derive(Debug, Serialize, Deserialize)]
pub struct Optimizer {
    /// The optimizer settings.
    settings: Settings,
}

impl Optimizer {
    /// A shortcut constructor.
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }

    /// Returns the optimizer settings reference.
    pub fn settings(&self) -> &Settings {
        &self.settings
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum OptimizationLevel {
    None = 0,
    Less = 1,
    Default = 2,
    Aggressive = 3,
}
