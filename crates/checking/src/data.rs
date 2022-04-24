// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use jup::tokens::Token;

use logging::logger::Logger;

/// Information for all checker structures
pub struct CheckerData {
    pub source: String,
    pub parsed: Vec<Vec<Token>>,
    pub logger: Logger,
    pub line_i: usize,
    pub token_i: usize,
}
