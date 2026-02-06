// SPDX-License-Identifier: GPL-3.0-only
// Execution context for RPN calculator
// Holds stack, heaps, and configuration state

use crate::error::Error;
use crate::number::NumberConfig;
use crate::stack::{Heap, RpnStack};

/// Execution context containing all runtime state
#[derive(Debug)]
pub struct Context {
    /// Main execution stack
    pub stack: RpnStack,

    /// Global heap for variables (sto/rcl)
    pub global_heap: Heap,

    /// Local heap for loop/function scope variables
    pub local_heap: Heap,

    /// Error context tracking (function name where error occurred)
    pub error_context: Option<String>,

    /// Last error that occurred (for error/strerror commands)
    pub last_error: Option<Error>,

    /// Flag to preserve last_error (set by error/strerror commands)
    pub preserve_last_error: bool,

    /// Number formatting and precision configuration
    pub config: NumberConfig,
}

impl Context {
    /// Create a new context with default configuration
    pub fn new() -> Self {
        Self {
            stack: RpnStack::new(),
            global_heap: Heap::new(),
            local_heap: Heap::new(),
            error_context: None,
            last_error: None,
            preserve_last_error: false,
            config: NumberConfig::default(),
        }
    }

    /// Clear error context
    pub fn clear_error_context(&mut self) {
        self.error_context = None;
    }

    /// Set error context
    pub fn set_error_context(&mut self, context: String) {
        self.error_context = Some(context);
    }

    /// Find a variable in local heap, then global heap
    /// Returns reference to object if found
    pub fn find_variable(&self, name: &str) -> Option<&crate::object::Object> {
        // First check local heap
        if let Some(obj) = self.local_heap.recall(name) {
            return Some(obj);
        }

        // Then check global heap
        self.global_heap.recall(name)
    }

    /// Find a mutable variable in local heap, then global heap
    pub fn find_variable_mut(&mut self, name: &str) -> Option<&mut crate::object::Object> {
        // Check local heap first
        if self.local_heap.contains(name) {
            return self.local_heap.recall_mut(name);
        }

        // Then check global heap
        self.global_heap.recall_mut(name)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
