//! TypeCmd - A TypeScript-like command line simulator
//!
//! This crate provides a command line interpreter with history support,
//! variable storage, and colorized output.

pub mod error;
pub mod colors;
pub mod history;
pub mod variables;
pub mod variablesint;
pub mod parser;
pub mod command;
pub mod executor;
pub mod repl;

// Re-export commonly used items
pub use error::{TypeCmdError, Result};
pub use history::HistoryManager;
pub use variables::VariableStore;
pub use command::{Command, ShowSubcommand, ClearTarget, HistorySubcommand};
pub use executor::TypeCmd;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::error::{TypeCmdError, Result};
    pub use crate::colors::{colorize, print_error, print_success, print_info, print_warn, print_gray, print_cyan};
    pub use crate::history::HistoryManager;
    pub use crate::variables::VariableStore;
    pub use crate::command::{Command, ShowSubcommand, ClearTarget, HistorySubcommand};
    pub use crate::executor::TypeCmd;
    pub use crate::parser::{parse_command, parse_to_command};
}