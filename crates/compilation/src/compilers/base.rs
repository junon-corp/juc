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
            operation::Operation,
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

    /// Walks through the given elements, calling `check` for each element
    ///
    /// Can skip an element if `is_skip_next` is true
    fn call(&mut self, elements: &Vec<Element>) {
        let mut i = 0;

        for element in elements {
            if self.data().is_skip_next {
                self.data().is_skip_next = false;
                i += 1;
                continue;
            }

            if i != elements.len() -1 {
                self.data().next_element = elements[i + 1].clone();
            }
            self.check(element);
            i += 1;
        }
    }

    /// Call to the right function according to the given element
    ///
    /// Note : It's not a logic or syntax checker, it only checks the element to
    /// call the right function
    fn check(&mut self, element: &Element) {
        match element.clone() {
            Element::Expression(elements) => self.call(&elements),
            Element::Function(function) => self.at_function(function),
            Element::Operation(operation) => match operation.operator() {
                Token::Assign => self.at_assign(&operation),
                Token::Plus => self.at_plus(&operation),
                _ => {}
            },
            Element::Return(token) => self.at_return(token),
            Element::Variable(variable) => self.at_variable(variable),
            Element::Other(token) => {}
        }
    }

    /// Executes the next expression before the one who call this function
    ///
    /// Note : It does not check if it's an expression or not, it take the
    /// next token in consideration as an expression
    fn execute_next_expression(&mut self) {
        let expression = self.data().next_element.clone();
        self.check(&expression);
        self.data().is_skip_next = true;
    }

    /// Link all generated files to one output file (library or binary according
    /// to the selected one)
    fn link(&mut self);

    /// Exit point
    ///
    /// Delete all temporary files and do linking
    fn finish(&mut self) {}

    /// Exit point for each source file
    fn finish_one(&mut self, source: &String);

    fn data(&mut self) -> &mut CompilerData;

    fn at_function(&mut self, function: Function);
    fn at_static(&mut self, variable: Variable);
    fn at_variable(&mut self, variable: Variable);

    fn at_assign(&mut self, operation: &Operation);
    fn at_plus(&mut self, operation: &Operation);

    fn at_return(&mut self, value: Token);

    fn assign_variable(&mut self, variable: &Variable);
}