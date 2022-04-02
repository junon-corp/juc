// This file is part of "juc"
// All rights reserved
// Copyright (c) Junon, Antonin Hérault

enum AsmToken {
    Call,
    Global,
    Jmp,
    Mov,
    Nop,
    Push,
    Pop,
    Section,
    SysCall,
    Register(AsmRegister)
}

enum AsmRegister {
    Rax,
    Rbp,
    Rdi,
    Rdx
}
