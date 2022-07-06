// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

pub mod manager;
pub mod structured;

use manager::StringsManager;

pub fn init_strings() -> StringsManager {
    let json_content = include_str!("../../../src/strings.json");
    StringsManager::from_string(json_content)
}
