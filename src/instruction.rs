#[derive(Copy, Clone, Debug, PartialEq)]
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
    ALOC,
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
            17 => Opcode::ALOC,
            _ => Opcode::IGL,
        }
    }
}

impl From<&str> for Opcode {
    fn from(s: &str) -> Self {
        match s {
            "HLT" => Opcode::HLT,
            "IGL" => Opcode::IGL,
            "LOAD" => Opcode::LOAD,
            "ADD" => Opcode::ADD,
            "SUB" => Opcode::SUB,
            "MUL" => Opcode::MUL,
            "DIV" => Opcode::DIV,
            "JMP" => Opcode::JMP,
            "JMPF" => Opcode::JMPF,
            "JMPB" => Opcode::JMPB,
            "EQ" => Opcode::EQ,
            "NEQ" => Opcode::NEQ,
            "GT" => Opcode::GT,
            "LT" => Opcode::LT,
            "GTQ" => Opcode::GTQ,
            "LTQ" => Opcode::LTQ,
            "JEQ" => Opcode::JEQ,
            "ALOC" => Opcode::ALOC,
            _ => Opcode::IGL,
        }
    }
}
