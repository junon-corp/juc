// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

pub mod base;
pub mod data;
pub mod syntax;

use crate::junon::compilation::checking::{
    base::Checker,
    data::CheckerData,
    syntax::SyntaxChecker,
};

pub fn run_checkers(data: CheckerData) {
    let mut syntax_checker = SyntaxChecker::new(data);
    syntax_checker.run();
}
