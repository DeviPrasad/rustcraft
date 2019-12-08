#[derive(Debug,PartialEq)]
pub enum Reg {
    R0,
    R1, R2, R3, R4, R5, R6, R7, R8, R9, R10,
    R11, R12, R13, R14, R15, R16, R17, R18, R19, R20,
    R21, R22, R23, R24, R25, R26, R27, R28, R29, R30,
    R31,

    SP,
    RA,

    Zero,
}


#[derive(Debug,PartialEq)]
pub enum Instruction {
    Mov(Reg, Reg),
    Add(Reg, Reg, Reg),
    AddI(Reg, Reg, i16),
    Sub(Reg, Reg, Reg),
    SubI(Reg, Reg, i16),

    Slt(Reg, Reg, Reg),
    SltU(Reg, Reg, Reg),
    SltImmU(Reg, Reg, i16),

    Beq(Reg, i16),
    Bneq(Reg, i16),
    Blt(Reg, i16),
    Bltu(Reg, i16),
    Bgt(Reg, i16),
    Bge(Reg, i16),

    Jmp(i16),
    Jal(Reg, i16),
    Jalr(Reg, i16),

    Call (i16),
    Return,

    Load(Reg, Reg, i16),
    Store(Reg, Reg, i16),

    Neg, // Two’s complement
    Not, // One’s complement
    Nop,
    Bad,
}
