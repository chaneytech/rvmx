#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    IGL,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    JEQ,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::HLT,
            1 => Opcode::IGL,
            2 => Opcode::LOAD,
            3 => Opcode::ADD,
            4 => Opcode::SUB,
            5 => Opcode::MUL,
            6 => Opcode::DIV,
            7 => Opcode::JMP,
            8 => Opcode::JMPF,
            9 => Opcode::JMPB,
            10 => Opcode::EQ,
            11 => Opcode::NEQ,
            12 => Opcode::GT,
            13 => Opcode::LT,
            14 => Opcode::GTQ,
            15 => Opcode::LTQ,
            16 => Opcode::JEQ,
            _ => Opcode::IGL,
        }
    }
}
