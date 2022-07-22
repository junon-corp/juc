// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use jup::lang::{
    elements::{
        function::Function,
        operation::Operation,
        variable::Variable,
        Element,
    },
    tokens::Token,
};

use crate::data::{
    CompilerData,
    CompilerTools,
    CompilerCodeData,
    CompilerStacksData,
};

/// Trait for a Compiler that will be followed by all platform's compilers.
///
/// Some functions are already defined because they are cross-platform.
///
/// The general documentation is written here to avoid to write the same
/// documentation to each platform's compiler. But a specific compiler can
/// have its own documentation.
pub trait Compiler {
    /// Global initialization.
    fn init(&mut self);
    /// Global termination.
    fn terminate(&mut self);

    /// Initialization for each source file.
    ///
    /// Has to run the tokenizer and parser to set `self.data().current_parsed`. 
    fn init_file(&mut self, source_path: &String);
    /// Termination for each source file.
    fn terminate_file(&mut self, source_path: &String);

    /// Runs the compiler by calling the initialization and termination 
    /// functions, then compile each source file before doing linkage.
    fn run(&mut self) {
        self.init();

        for source_path in self.data().sources.clone() {
            // todo!() : Setting the current scope as the source path 
            // (considering its folder and filename).

            self.init_file(&source_path);
            
            // Executes calls for the parsed elements from the source file after
            // having parsed it in `init_file()`.
            let source_elements = self.code_data().current_parsed.clone();
            self.call_for_elements(&source_elements);

            self.terminate_file(&source_path);
        }

        self.link();
        self.terminate();
    }

    /// Links generated files to one output file
    fn link(&mut self);

    /// Walks through the given elements, calling `check` for each element.
    ///
    /// Skips an element when `is_skip_next` is true.
    fn call_for_elements(&mut self, elements: &Vec<Element>) {
        let mut i = 0;

        for element in elements {
            if self.code_data().is_skip_next {
                self.code_data().is_skip_next = false;
                i += 1;
                continue;
            }

            if i != elements.len() -1 {
                self.code_data().next_element = elements[i + 1].clone();
            }
            self.check_element(element);
            i += 1;
        }
    }

    /// Calls to the right function according to the given element.
    ///
    /// Note : It's not a logic or syntax checker, it only checks the element to
    /// call the right function.
    fn check_element(&mut self, element: &Element) {
        // All cases where the element raises no call is because we don't care 
        // about it because we will care it in another function called 
        // because of another element.

        match element {
            Element::Array(_values) => {},
            Element::Assembly(code) => self.at_assembly(code),
            // Redo a call loop for the elements in the expression found
            Element::Expression(elements) => self.call_for_elements(elements),
            Element::Function(function) => self.at_function(function),
            Element::Operation(operation) => self.at_operation(operation),
            Element::Return(value) => self.at_return(value),
            Element::Parameters(_tokens) => {}
            Element::Variable(variable) => self.at_variable(variable),
            Element::Other(token) => self.at_other(token),
        }
    }

    // Data getters as it's required -------------------------------------------
    //
    // If the getters are not implemented here it's because they cannot, a 
    // trait does not embed values within it.

    fn data(&mut self) -> &mut CompilerData;
    fn tools(&mut self) -> &mut CompilerTools;
    fn code_data(&mut self) -> &mut CompilerCodeData;
    fn stacks_data(&mut self) -> &mut CompilerStacksData;

    // Functions for the elements ----------------------------------------------

    /// Writes the Assembly code contained into the `code` token in the output
    /// file.
    fn at_assembly(&mut self, code: &Token); 

    /// Adds a function based on the given object
    fn at_function(&mut self, function: &Function);

    // Checks the right kind of operation to call the right operation
    // function
    fn at_operation(&mut self, operation: &Operation) {
        match operation.operator() {
            Token::Assign => self.at_assign(&operation),
            Token::Plus => self.at_plus(&operation),
            Token::Minus => self.at_minus(&operation),
            Token::Multiply => self.at_multiply(&operation),
            Token::Divide => self.at_divide(&operation),
            _ => panic!("invalid operation for operation : {:?}", operation),
        }
    }

    fn at_assign(&mut self, operation: &Operation);
    fn at_plus(&mut self, operation: &Operation);
    fn at_minus(&mut self, operation: &Operation);
    fn at_multiply(&mut self, operation: &Operation);
    fn at_divide(&mut self, operation: &Operation);

    fn at_return(&mut self, value: &Token);

    fn at_variable(&mut self, variable: &Variable);

    fn at_other(&mut self, other: &Token) {
        match other {
            Token::NewLine => {},
            Token::Other(id_or_value) => {
                if self.stacks_data().variable_stack.get(id_or_value).is_none() {
                    // No variable for this identifier was found, so we call
                    // its associated function
                    self.call_function(id_or_value);
                } else {
                    self.update_return_register(other);
                }
            }
            _ => panic!("unknown token : {:?}", other),
        }
    }

    // Other functions for Assembly code ---------------------------------------

    fn call_function(&mut self, id: &String);
    fn update_return_register(&mut self, value: &Token);
}
