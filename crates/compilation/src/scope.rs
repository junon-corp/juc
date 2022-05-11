// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::string::ToString;

use crate::defaults;

/// Simple structure to manage a scope, an indicator to where we are in the
/// junon program \
/// Implements a `to_string()` function to create a string with all sub scopes
/// separated by the defaults scope separator, that is why a simple Vec is not
/// used.
#[derive(Clone)]
pub struct Scope {
    scope: Vec<String>,
}

impl ToString for Scope {
    fn to_string(&self) -> String {
        let mut to_write = String::new();
        for (i, sub_scope) in self.scope.iter().enumerate() {
            to_write += sub_scope;
            if i != self.scope.len() - 1 {
                to_write += defaults::SCOPE_SEPARATOR
            }
        }
        to_write
    }
}

impl Scope {
    pub fn new() -> Self {
        Self { scope: vec![] }
    }

    pub fn from(start: Vec<String>) -> Self {
        Self { scope: start }
    }

    pub fn push(&mut self, sub_scope: String) {
        self.scope.push(sub_scope);
    }

    pub fn pop(&mut self) {
        self.scope.pop();
    }

    pub fn reset(&mut self) {
        self.scope = vec![];
    }
}
