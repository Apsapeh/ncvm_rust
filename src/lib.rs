mod ncvm;

pub use ncvm::{
    opcode, 
    Instruction,
    Instruction_LongOrDouble
};

use ncvm::opcode::OPCODE;

impl ncvm::Instruction {
    pub fn new(
        opcode: OPCODE,
        r1: ::std::os::raw::c_uchar,
        r2: ::std::os::raw::c_uchar,
        r3: Instruction_LongOrDouble
    ) -> Instruction {
        Instruction {
            opcode, r1, r2, r3
        }
    }

    pub fn new_f(
        opcode: OPCODE,
        r1: ::std::os::raw::c_uchar,
        r2: ::std::os::raw::c_uchar,
        r3: f64
    ) -> Instruction {
        Instruction {
            opcode, r1, r2, 
            r3: Instruction_LongOrDouble {valf: r3}
        }
    }

    pub fn new_i(
        opcode: OPCODE,
        r1: ::std::os::raw::c_uchar,
        r2: ::std::os::raw::c_uchar,
        r3: ::std::os::raw::c_ulonglong
    ) -> Instruction {
        Instruction {
            opcode, r1, r2, 
            r3: Instruction_LongOrDouble {vali: r3}
        }
    }
}

impl ncvm::ncvm {
    pub fn new() {
        
    }
}