use std::collections::VecDeque;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::error::{TypeCmdError, Result};

/// History entry structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: u64,
    pub command: String,
    pub timestamp: DateTime<Local>,
}

/// Configuration for history manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryConfig {
    pub max_history_size: usize,
    pub history_file: String,
    pub version: String,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            max_history_size: 1000,
            history_file: ".typecmd_history".to_string(),
            version: "0.4.0".to_string(),
        }
    }
}

/// History manager for storing and retrieving command history
pub struct HistoryManager {
    entries: VecDeque<HistoryEntry>,
    next_id: u64,
    config: HistoryConfig,
}

impl HistoryManager {
    /// Create a new history manager with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(HistoryConfig::default())
    }
    
    /// Create a new history manager with custom configuration
    pub fn with_config(config: HistoryConfig) -> Result<Self> {
        let mut manager = HistoryManager {
            entries: VecDeque::new(),
            next_id: 1,
            config,
        };
        
        manager.load_history()?;
        Ok(manager)
    }
    
    /// Get the path to the history file
    fn history_path(&self) -> PathBuf {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(&self.config.history_file)
    }
    
    /// Load history from file
    fn load_history(&mut self) -> Result<()> {
        let path = self.history_path();
        
        if !path.exists() {
            return Ok(());
        }
        
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = line.splitn(3, '|').collect();
            if parts.len() == 3 {
                if let (Ok(id), Ok(timestamp)) = (
                    parts[0].parse::<u64>(),
                    parts[1].parse::<DateTime<Local>>()
                ) {
                    let entry = HistoryEntry {
                        id,
                        command: parts[2].to_string(),
                        timestamp,
                    };
                    
                    self.entries.push_back(entry);
                    self.next_id = self.next_id.max(id + 1);
                }
            }
        }
        
        // Limit history size
        while self.entries.len() > self.config.max_history_size {
            self.entries.pop_front();
        }
        
        Ok(())
    }
    
    /// Save history to file
    fn save_history(&self) -> Result<()> {
        let path = self.history_path();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)?;
        
        for entry in &self.entries {
            writeln!(
                file,
                "{}|{}|{}",
                entry.id,
                entry.timestamp.to_rfc3339(),
                entry.command
            )?;
        }
        
        Ok(())
    }
    
    /// Add a command to history
    pub fn add(&mut self, command: &str) -> Result<()> {
        if command.trim().is_empty() {
            return Ok(());
        }
        
        // Skip history commands themselves
        if command.starts_with("history") || command.starts_with('!') {
            return Ok(());
        }
        
        let entry = HistoryEntry {
            id: self.next_id,
            command: command.to_string(),
            timestamp: Local::now(),
        };
        
        self.entries.push_back(entry);
        self.next_id += 1;
        
        // Limit size
        while self.entries.len() > self.config.max_history_size {
            self.entries.pop_front();
        }
        
        self.save_history()?;
        Ok(())
    }
    
    /// Get history entries, optionally limited by count
    pub fn get(&self, limit: Option<usize>) -> Vec<HistoryEntry> {
        match limit {
            Some(l) => self.entries.iter().rev().take(l).cloned().collect(),
            None => self.entries.iter().cloned().collect(),
        }
    }
    
    /// Get a history entry by ID
    pub fn get_by_id(&self, id: u64) -> Option<&HistoryEntry> {
        self.entries.iter().find(|e| e.id == id)
    }
    
    /// Search history entries by keyword
    pub fn search(&self, keyword: &str) -> Vec<&HistoryEntry> {
        let keyword_lower = keyword.to_lowercase();
        self.entries
            .iter()
            .filter(|e| e.command.to_lowercase().contains(&keyword_lower))
            .collect()
    }
    
    /// Clear all history
    pub fn clear(&mut self) -> Result<()> {
        self.entries.clear();
        self.next_id = 1;
        let _ = fs::remove_file(self.history_path());
        Ok(())
    }
    
    /// Get the last history entry
    pub fn last(&self) -> Option<&HistoryEntry> {
        self.entries.back()
    }
    
    /// Get the count of history entries
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    
    /// Get configuration
    pub fn config(&self) -> &HistoryConfig {
        &self.config
    }
}