// This file is part of "juc"
// Under the MIT License
// Copyright (c) Junon, Antonin Hérault

use x64asm::register::Register;

pub const ENTRY_POINT: &str = "main";
pub const EXTENSION: &str = "ju";
pub const EXTENSION_COMPLETE: &str = ".ju";
pub const SCOPE_SEPARATOR: &str = ".";
pub const FUN_RETURN_REGISTER: Register = Register::Rax;
pub const RETURN_REGISTER: Register = Register::Rbx;
pub const RETURN_REGISTER_2: Register = Register::Rdx;

pub mod linux_defaults {
    pub const ASSEMBLER: &str = "nasm";
    pub const LINKER: &str = "ld";

    pub const OUTPUT_FILE: &str = "junon.out";
}
