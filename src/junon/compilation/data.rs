// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

use std::fs::File;
use std::collections::HashMap as Dict;

/// Important information given to the compiler
pub struct CompilerData {
    pub sources: Vec<String>, 
    pub options: Dict<String, String>,
    pub is_library: bool,
    pub stream: Option<File>,
}
