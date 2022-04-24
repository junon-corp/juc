// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

pub mod base;
pub mod data;
pub mod syntax;

use logging::logger::Logger;

use crate::base::Checker;

pub fn run_checkers(data: data::CheckerData) -> Result<(), Logger> {
    let mut syntax_checker = syntax::SyntaxChecker::new(data);
    syntax_checker.run()?;

    Ok(())
}
