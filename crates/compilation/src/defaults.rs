// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use x64asm::register::Register;

pub const BUILD_FOLDER: &str = ".junon";
pub const ENTRY_POINT: &str = "main";
pub const EXTENSION: &str = "ju";
pub const EXTENSION_COMPLETE: &str = ".ju";
pub const SCOPE_SEPARATOR: &str = ".";
pub const RETURN_REGISTER: Register = Register::Rax;

pub mod linux_defaults {
    pub const ASSEMBLER: &str = "nasm";
    pub const LINKER: &str = "ld";

    pub const OUTPUT_FILE: &str = "junon.out";

    pub const START_FILE: &str = "startju.asm";
    pub const START_FUNCTION: &str = "_start";
}
