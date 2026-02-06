// SPDX-License-Identifier: GPL-3.0-only
// Stack and Heap data structures

use crate::object::Object;
use std::collections::{HashMap, VecDeque};

/// RPN stack using front-based operations (LIFO)
#[derive(Debug, Clone)]
pub struct RpnStack {
    data: VecDeque<Object>,
}

impl RpnStack {
    /// Create a new empty stack
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    /// Push object to front of stack (top)
    pub fn push(&mut self, obj: Object) {
        self.data.push_front(obj);
    }

    /// Pop object from front of stack (top)
    pub fn pop(&mut self) -> Option<Object> {
        self.data.pop_front()
    }

    /// Get reference to object at level (0 = top of stack)
    pub fn get(&self, level: usize) -> Option<&Object> {
        self.data.get(level)
    }

    /// Get mutable reference to object at level
    pub fn get_mut(&mut self, level: usize) -> Option<&mut Object> {
        self.data.get_mut(level)
    }

    /// Get stack length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if stack is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Erase elements from stack
    /// first: index of first element to erase
    /// count: number of elements to erase
    pub fn erase(&mut self, first: usize, count: usize) {
        let last = (first + count).min(self.data.len());
        self.data.drain(first..last);
    }

    /// Clear all elements from stack
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get object type name at level
    pub fn type_name(&self, level: usize) -> Option<&'static str> {
        self.get(level).map(|obj| obj.type_name())
    }

    /// Get iterator over stack (from top to bottom)
    pub fn iter(&self) -> impl Iterator<Item = &Object> {
        self.data.iter()
    }
}

impl Default for RpnStack {
    fn default() -> Self {
        Self::new()
    }
}

/// Heap for variable storage (name -> object mapping)
#[derive(Debug, Clone)]
pub struct Heap {
    vars: HashMap<String, Object>,
}

impl Heap {
    /// Create a new empty heap
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    /// Store an object with given name
    pub fn store(&mut self, name: String, obj: Object) {
        self.vars.insert(name, obj);
    }

    /// Recall (get) an object by name
    pub fn recall(&self, name: &str) -> Option<&Object> {
        self.vars.get(name)
    }

    /// Recall (get) a mutable reference to an object by name
    pub fn recall_mut(&mut self, name: &str) -> Option<&mut Object> {
        self.vars.get_mut(name)
    }

    /// Purge (remove) an object by name, returning it if it existed
    pub fn purge(&mut self, name: &str) -> Option<Object> {
        self.vars.remove(name)
    }

    /// Clear all variables
    pub fn clear(&mut self) {
        self.vars.clear();
    }

    /// Get all variable names
    pub fn var_names(&self) -> Vec<&String> {
        self.vars.keys().collect()
    }

    /// Check if a variable exists
    pub fn contains(&self, name: &str) -> bool {
        self.vars.contains_key(name)
    }

    /// Get number of variables
    pub fn len(&self) -> usize {
        self.vars.len()
    }

    /// Check if heap is empty
    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}
