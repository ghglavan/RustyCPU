use crate::constants::*;

pub enum Instructions {
   ThreeArgs(OpCodes, Registers, Registers, Registers),
   TwoArgs(OpCodes, Registers, Address),
   OneArg(OpCodes, Address),
   NoArgs(OpCodes)
}

impl Instructions {
    pub fn opcode(&self) -> &OpCodes {
        match self {
            Self::NoArgs(op) | Self::OneArg(op, _) | Self::TwoArgs(op, _, _) | Self::ThreeArgs(op, _, _, _) => op
        }
    }
    pub fn r1(&self) -> &Registers {
        match self {
            Self::TwoArgs(_, r1, _) | Self::ThreeArgs(_, r1, _, _) => r1,
            _ => panic!("no r1 in instruction") // maybe we should handle the error?..
        }
    }

    pub fn r2(&self) -> &Registers {
        match self {
            Self::ThreeArgs(_, _, r2, _) => r2,
            _ => panic!("no r2 in instruction") // maybe we should handle the error?..
        }
    }

    pub fn r3(&self) -> &Registers {
        match self {
            Self::ThreeArgs(_, _, _, r3) => r3,
            _ => panic!("no r3 in instruction") // maybe we should handle the error?..
        }
    }

    pub fn addr(&self) -> &Address {
        match self {
            Self::TwoArgs(_, _, addr) | Self::OneArg(_, addr) => addr,
            _ => panic!("no address in instruction") // maybe we should handle the error?..
        }
    }
}

