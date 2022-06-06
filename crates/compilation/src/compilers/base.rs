// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::path::Path;

use jup::{
    checking::syntax::SyntaxChecker, 
    lang::{
        elements::{ 
            Element, 
            function::Function, 
            type_::Type, 
            variable::Variable
        },
        tokens::Token,
    },
    parser::Parser,
    tokenizer::Tokenizer,
};

use crate::{
    data::CompilerData, 
    defaults, 
    scope::Scope
};

/// Trait for a Compiler followed by all platform's compilers \
/// Some functions are already defined because they are cross-platform \
/// The general documentation is written here to avoid to write the same
/// documentation to each platform's compilers. But a specific compiler can
/// have its own documentation
pub trait Compiler {
    /// Starting point \
    /// Do some stuff useful
    fn init(&mut self);

    /// Starting point for each source file
    fn init_one(&mut self, source: &String) {
        self.data().current_source = format!("{}/{}.asm", defaults::BUILD_FOLDER, source);

        let mut tokenizer = Tokenizer::from_path(Path::new(source)).unwrap();
        tokenizer.run();

        let mut parser = Parser::new(tokenizer.tokenized().clone());
        parser.run();

        println!("{:#?}", parser.parsed());

        self.data().current_parsed = parser.parsed().clone();

        // TODO : Run the syntax checker here
    }

    /// Main function where each source file is transformed to an objet file
    fn run(&mut self) {
        self.init();

        // NOTE : If any syntax problem is found during syntax checking, the
        // program will be terminated. They should be retrieved and printed
        // after all sources checks
        // TODO : Update "jup" according to the previous NOTE
        for source in self.data().sources.clone() {
            // Module name is the filename without the ".ju" extension
            self.data().current_scope = Scope::from(vec![format!("{}", source)
                .split(defaults::EXTENSION_COMPLETE)
                .collect::<String>()]);

            self.init_one(&source);
            
            let mut current_parsed = self.data().current_parsed.clone();

            self.call(&current_parsed);

            self.finish_one(&source);
        }

        self.link();
        self.finish();
    }

    /// Methods caller according to the current token
    fn call(&mut self, elements: &Vec<Element>) {
        for element in elements {
            self.check(element);
        }
    }

    /// Returns how much elements should be skip
    fn check(&mut self, element: &Element) {
        println!("{:?}", element);

        match element.clone() {
            Element::Expression(elements) => self.call(&elements),
            Element::Function(function) => self.add_function(function),
            Element::Operation(operation) => {},
            Element::Return(token) => self.return_(token),
            Element::Variable(variable) => self.add_variable(variable),
            Element::Other(token) => {}
        }
    }

    /// Link all generated files to one output file (library or binary according
    /// to the selected one)
    fn link(&mut self);

    /// Exit point \
    /// Delete all temporary files and do linking
    fn finish(&mut self) {}

    /// Exit point for each source file
    fn finish_one(&mut self, source: &String);

    /// Data getter
    fn data(&mut self) -> &mut CompilerData;

    // --- ASM code generators

    fn add_variable(&mut self, variable: Variable);
    fn add_static_variable(&mut self, variable: Variable);
    fn add_function(&mut self, function: Function);

    fn assign_variable(&mut self, variable: &Variable);

    fn return_(&mut self, value: Token);
}
