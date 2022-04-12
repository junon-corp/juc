// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

pub mod compilation;

pub mod args;
pub mod logger;
pub mod platform;

/// Usage : `use crate::junon::Dict;`
pub type Dict<K, V> = std::collections::HashMap<K, V>;
