use std::collections::HashMap;

/// Variable storage manager
#[derive(Debug, Clone, Default)]
pub struct VariableStoreInt {
    store: HashMap<String, i64>,
}

impl VariableStoreInt {
    /// Create a new empty variable store
    pub fn new() -> Self {
        VariableStoreInt {
            store: HashMap::new(),
        }
    }
    
    /// Get a variable value
    pub fn get(&self, key: &str) -> Option<&i64> {
        self.store.get(key)
    }
    
    /// Set a variable value
    pub fn set(&mut self, key: String, value: i64) {
        self.store.insert(key, value);
    }
    
    /// Check if a variable exists
    pub fn has(&self, key: &str) -> bool {
        self.store.contains_key(key)
    }
    
    /// Delete a variable
    pub fn delete(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }
    
    /// Clear all variables
    pub fn clear(&mut self) {
        self.store.clear();
    }
    
    /// Get all variables
    pub fn all(&self) -> &HashMap<String, i64> {
        &self.store
    }
    
    /// Get the number of variables
    pub fn len(&self) -> usize {
        self.store.len()
    }
    
    /// Check if the store is empty
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}