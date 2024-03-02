#![allow(dead_code)]

// At this moment, it's a full implementation

mod clib_ncvm;


pub use clib_ncvm::{
    opcode, 
    register,
    Instruction,
    Instruction_LongOrDouble,
    ThreadSettings
};

impl clib_ncvm::Instruction {
    pub fn new(
        opcode: opcode::OPCODE,
        r1: ::std::os::raw::c_uchar,
        r2: ::std::os::raw::c_uchar,
        r3: Instruction_LongOrDouble
    ) -> Instruction {
        Instruction {
            opcode, r1, r2, r3
        }
    }

    pub fn new_f(
        opcode: opcode::OPCODE,
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
        opcode: opcode::OPCODE,
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

impl ThreadSettings {
    pub fn new(
        u32_reg_size: usize, 
        u64_reg_size: usize,
        f32_reg_size: usize,
        f64_reg_size: usize,
        stack_size: usize
    ) -> ThreadSettings {
        ThreadSettings {
            u32_reg_size: u32_reg_size as ::std::os::raw::c_ulong, 
            u64_reg_size: u64_reg_size as ::std::os::raw::c_ulong,
            f32_reg_size: f32_reg_size as ::std::os::raw::c_ulong,
            f64_reg_size: f64_reg_size as ::std::os::raw::c_ulong,
            stack_size:     stack_size as ::std::os::raw::c_ulong
        }
    }

    pub fn default() -> ThreadSettings {
        ThreadSettings {
            u32_reg_size: 8, 
            u64_reg_size: 8,
            f32_reg_size: 8,
            f64_reg_size: 8,
            stack_size:   1024*1024*1, // 1 MiB
        }
    }
}

pub struct VM {
    c_vm: clib_ncvm::ncvm
}

impl VM {
    pub fn new(
        instructions: &mut Vec<Instruction>,
        static_memory: &mut Vec<::std::os::raw::c_uchar>
    ) -> VM {
        unsafe { VM {
            c_vm: clib_ncvm::ncvm_initArr(
                instructions.as_mut_ptr(), 
                instructions.len() as ::std::os::raw::c_ulong,
                static_memory.as_mut_ptr(),
                static_memory.len() as ::std::os::raw::c_ulong,
            )
        }}
    }

    pub fn from_bin(mut bin_data: Vec<::std::os::raw::c_char>) -> VM {
        unsafe { VM {
            c_vm: clib_ncvm::ncvm_initData(
                bin_data.as_mut_ptr(), 
                bin_data.len() as ::std::os::raw::c_ulong,
            )
        }}
    }

    pub fn execute(&mut self, settings: ThreadSettings) {
        unsafe {
            clib_ncvm::ncvm_execute(&mut self.c_vm, settings);
        }
    }

    pub fn create_thread(
        &mut self, start_instruction_index: usize,
        ext_stack: &mut Vec<::std::os::raw::c_uchar>,
        settings: ThreadSettings 
    ) {
        unsafe {
            clib_ncvm::ncvm_create_thread(
                &mut self.c_vm,
                self.c_vm.inst_p.add(start_instruction_index),
                ext_stack.as_mut_ptr(),
                ext_stack.len() as ::std::os::raw::c_ulong,
                settings
            );
        }
    }
}