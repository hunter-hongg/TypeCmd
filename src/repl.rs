//! REPL (Read-Eval-Print Loop) module for interactive command execution
//! This module provides a higher-level interface for running TypeCmd

use crate::error::Result;
use crate::executor::TypeCmd;

/// REPL runner for TypeCmd
pub struct Repl {
    typecmd: TypeCmd,
}

impl Repl {
    /// Create a new REPL instance
    pub fn new() -> Result<Self> {
        Ok(Repl {
            typecmd: TypeCmd::new()?,
        })
    }
    
    /// Run the REPL
    pub fn run(&mut self) -> Result<()> {
        self.typecmd.run()
    }
    
    /// Execute a single command
    pub fn execute(&mut self, command: &str) -> Result<Option<String>> {
        self.typecmd.execute_command(command)
    }
    
    /// Get a reference to the underlying TypeCmd instance
    pub fn typecmd(&self) -> &TypeCmd {
        &self.typecmd
    }
    
    /// Get a mutable reference to the underlying TypeCmd instance
    pub fn typecmd_mut(&mut self) -> &mut TypeCmd {
        &mut self.typecmd
    }
}