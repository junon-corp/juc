// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use std::{
    fs,
    path::Path,
};

use crate::structured::*;

pub struct StringsManager {
    deserialized: Strings,
}

impl StringsManager {
    pub fn new(deserialized: Strings) -> Self {
        Self {
            deserialized
        }
    }

    pub fn from_path(path: &Path) -> Self {
        let json_content = fs::read_to_string(path).unwrap();
        
        Self {
            deserialized: serde_json::from_str(&json_content).unwrap()
        }
    }

    pub fn get<T>(&self, element: &str) -> &T {
        let splitted: Vec<&str> = element.split(".").collect();
        
        let r: StringsObject = match splitted[0] {
            "help" => {
                StringsObject::Help(&self.deserialized.help)
            },
            "logs" => {
                StringsObject::Logs(&self.deserialized.logs)
            }
            _ => panic!()
        };

        r.retrieve_object::<T>()
    }
}

/// A variant type to retrieve an object from module `structured`
enum StringsObject<'a> {
    Help(&'a Help),
    HelpArguments(&'a HelpArguments),
    HelpAvailableFlags(&'a HelpAvailableFlags),
    Logs(&'a Logs),
    Infos(&'a Infos),
    Errors(&'a Errors),
    Log(&'a Log),
    MultiString(&'a MultiString),
}

impl<'a> StringsObject<'a> {
    fn retrieve_object<T>(&self) -> &T {
        match *self {
            StringsObject::Help(object) => object,
            StringsObject::HelpArguments(object) => object,
            StringsObject::HelpAvailableFlags(object) => object,
            StringsObject::Logs(object) => object,
            StringsObject::Infos(object) => object,
            StringsObject::Errors(object) => object,
            StringsObject::Log(object) => object,
            StringsObject::MultiString(object) => object,
        }
    }
}

#[test]
fn strings_manager() {
    let path = Path::new("../../src/strings.json");
    let strings = StringsManager::from_path(&path);
    println!("{:#?}", strings.get::<Logs>("logs"));
}
