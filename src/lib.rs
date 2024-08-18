/*#![allow(dead_code)]

// At this moment, it's a full implementation

mod clib_ncvm;


pub use clib_ncvm::{
    Opcode,
    Register,
    Instruction,
    Instruction_LongOrDouble,
    ThreadSettings
};

pub use clib_ncvm::OPCODE as opcode_type;
pub use clib_ncvm::Register as register_type;

impl clib_ncvm::Instruction {
    pub fn new(
        opcode: Opcode,
        r1: ::std::os::raw::c_uchar,
        r2: ::std::os::raw::c_uchar,
        r3: Instruction_LongOrDouble
    ) -> Instruction {
        Instruction {
            opcode: opcode as opcode_type, r1, r2, r3
        }
    }

    pub fn new_f(
        opcode: Opcode,
        r1: ::std::os::raw::c_uchar,
        r2: ::std::os::raw::c_uchar,
        r3: f64
    ) -> Instruction {
        Instruction {
            opcode: opcode as opcode_type, r1, r2,
            r3: Instruction_LongOrDouble {valf: r3}
        }
    }

    pub fn new_i(
        opcode: Opcode,
        r1: ::std::os::raw::c_uchar,
        r2: ::std::os::raw::c_uchar,
        r3: ::std::os::raw::c_ulonglong
    ) -> Instruction {
        Instruction {
            opcode: opcode as opcode_type, r1, r2,
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
}*/

use std::{
    collections::HashMap,
    ptr::{null, null_mut},
};

use clib_ncvm::Instruction;

pub mod clib_ncvm;

pub use ncvm_derive::native_function;

enum PtrOrValue<T> {
    Ptr(*mut T),
    Value(T),
}

pub struct VM {
    pub c_vm: clib_ncvm::ncvm,
}

pub struct Thread {
    c_thread: PtrOrValue<clib_ncvm::ncvm_thread>,
}

pub type NativeFunction = unsafe extern "C" fn(*mut clib_ncvm::ncvm_thread);

pub enum NativeFunctions {
    Files(Vec<String>),
    Functions(HashMap<String, NativeFunction>),
    //CustomGetter(fn(name: &str, data: &dyn std::any::Any) -> NativeFunction),
}

struct DataPtrWrap {
    ptr: *const u8,
}

impl VM {
    /*pub fn new(instructions: &mut Vec<Instruction>, static_memory: &mut Vec<u8>, native_lib_getter: fn(name: &str)) -> VM {
        let mut vm = std::mem::MaybeUninit::uninit();
        unsafe {
            /*let settings = clib_ncvm::ThreadSettings {
                u32_reg_size: 8,
                u64_reg_size: 8,
                f32_reg_size: 8,
                f64_reg_size: 8,
                stack_size: 1024 * 1024 * 1, // 1 MiB
                call_stack_size: 1024 * 1024 * 1,
            };
            vm = clib_ncvm::ncvm {
                inst_p: null_mut(),
                static_mem_p: null_mut(),
                inst_count: 0,
                static_mem_size: 0,
                main_thread_settings: settings,
                lib_functions: null_mut(),
            };*/
            clib_ncvm::ncvm_init(vm.as_mut_ptr(), instructions.as_mut_ptr(), static_memory.as_mut_ptr(), None, null_mut());
        }
        let vm = unsafe { vm.assume_init() };

        return VM {c_vm: vm};
    }*/
    pub fn from_file(path: &str) -> () {}

    unsafe extern "C" fn get_next_n_bytes(
        n: ::std::os::raw::c_ulong,
        data_p: *mut ::std::os::raw::c_void,
    ) -> *const u8 {
        let data = &mut *(data_p as *mut DataPtrWrap);

        let return_ptr = data.ptr;
        data.ptr = data.ptr.add(n as usize);
        //data_p = data_p.add(n as usize);
        return_ptr
    }

    unsafe extern "C" fn empty_lib_getter(
        name: *const ::std::os::raw::c_char,
        data_p: *mut ::std::os::raw::c_void,
    ) -> clib_ncvm::ncvm_lib_function {
        None
    }

    unsafe extern "C" fn rust_lib_getter(
        name: *const ::std::os::raw::c_char,
        data_p: *mut ::std::os::raw::c_void,
    ) -> clib_ncvm::ncvm_lib_function {
        let data = data_p as *mut HashMap<String, NativeFunction>;
        let name = std::ffi::CStr::from_ptr(name).to_str().unwrap();

        println!("rust_lib_getter: {}", name);
        println!("data_p: {:?}", *data);

        match (*data).get(name) {
            Some(f) => Some(*f),
            None => None,
        }
    }

    /*unsafe extern "C" fn get_rust_native_function(
        name: *const ::std::os::raw::c_char,
        data_p: *mut ::std::os::raw::c_void,
    ) -> clib_ncvm::ncvm_lib_function {
        let native_fns = &*(data_p as *const HashMap<String, NativeFunction>);
        let name = std::ffi::CStr::from_ptr(name).to_str().unwrap();

    }*/

    pub fn from_data(
        mut bytecode: Vec<u8>,
        native_fns: Option<NativeFunctions>,
    ) -> Result<VM, String> {
        unsafe {
            //a(null_mut());
            let mut data_ptr = DataPtrWrap {
                ptr: bytecode.as_ptr(),
            };

            let mut vm = std::mem::MaybeUninit::uninit();
            let ptr_vm = vm.as_mut_ptr();
            let gn = Some(
                VM::get_next_n_bytes
                    as unsafe extern "C" fn(
                        ::std::os::raw::c_ulong,
                        *mut ::std::os::raw::c_void,
                    ) -> *const u8,
            );
            let gl = Some(
                clib_ncvm::ncvm_default_get_lib_function
                    as unsafe extern "C" fn(
                        *const ::std::os::raw::c_char,
                        *mut ::std::os::raw::c_void,
                    ) -> clib_ncvm::ncvm_lib_function,
            );

            let libs = vec!["/home/ghost/Desktop/Rust/Projects/ncvm_rust/liblib1.so\0"];

            let l = libs.as_ptr() as *mut *const ::std::os::raw::c_char;
            let mut lib_loader =
                clib_ncvm::ncvm_default_lib_loader_init(l, libs.len() as ::std::os::raw::c_ulong);

            let (lib_getter, 
                lib_getter_data) = match native_fns {
                Some(NativeFunctions::Files(files)) => {
                    let mut cstr_libs = files
                        .into_iter()
                        .map(|s| std::ffi::CString::new(s).unwrap().into_raw() as *const i8)
                        .collect::<Vec<_>>();
                    let mut lib_loader = clib_ncvm::ncvm_default_lib_loader_init(
                        cstr_libs.as_mut_ptr(),
                        cstr_libs.len() as ::std::os::raw::c_ulong,
                    );
                    (
                        Some(
                            clib_ncvm::ncvm_default_get_lib_function
                                as unsafe extern "C" fn(
                                    *const ::std::os::raw::c_char,
                                    *mut ::std::os::raw::c_void,
                                )
                                    -> clib_ncvm::ncvm_lib_function,
                        ),
                        &mut lib_loader as *mut clib_ncvm::ncvm_default_lib_loader
                            as *mut ::std::os::raw::c_void,
                    )
                }
                Some(NativeFunctions::Functions(mut functions)) => {
                    println!("functions: {:?}", functions);
                    (Some(
                        VM::rust_lib_getter
                            as unsafe extern "C" fn(
                                *const ::std::os::raw::c_char,
                                *mut ::std::os::raw::c_void,
                            )
                                -> clib_ncvm::ncvm_lib_function,
                    ),
                    &mut functions as *mut HashMap<String, NativeFunction> as *mut ::std::os::raw::c_void)
                },
                None => (
                    Some(
                        VM::empty_lib_getter
                            as unsafe extern "C" fn(
                                *const ::std::os::raw::c_char,
                                *mut ::std::os::raw::c_void,
                            )
                                -> clib_ncvm::ncvm_lib_function,
                    ),
                    null_mut(),
                ),
            };

            println!("data: {:?}", lib_getter_data as *mut HashMap<String, NativeFunction> as &mut HashMap<String, NativeFunction>);

            let r = clib_ncvm::ncvm_loadBytecodeStream(
                ptr_vm,
                gn,
                &mut data_ptr as *mut DataPtrWrap as *mut ::std::os::raw::c_void,
                lib_getter,
                //&mut lib_loader as *mut clib_ncvm::ncvm_default_lib_loader as *mut ::std::os::raw::c_void,
                lib_getter_data,
            );

            println!("R: {} {}", r, clib_ncvm::NCVM_LIB_FUNCTION_NOT_FOUND);

            Ok(VM {
                c_vm: vm.assume_init(),
            })
        }
    }
}

// Threade functions
impl VM {
    pub fn create_thread(
        &mut self,
        start_instruction_index: usize,
        ext_stack: Option<&mut Vec<u8>>,
        settings: Option<clib_ncvm::ThreadSettings>,
    ) -> Thread {
        let mut thread = std::mem::MaybeUninit::uninit();
        unsafe {
            let (ext_stack_ptr, ext_stack_len) = match ext_stack {
                Some(stack) => (stack.as_mut_ptr(), stack.len() as ::std::os::raw::c_ulong),
                None => (null_mut(), 0),
            };
            let settings = match settings {
                Some(settings) => settings,
                None => self.c_vm.main_thread_settings,
            };
            clib_ncvm::ncvm_create_thread(
                thread.as_mut_ptr(),
                &mut self.c_vm,
                self.c_vm.inst_p.add(start_instruction_index),
                ext_stack_ptr,
                ext_stack_len,
                settings,
            );
        }
        let thread = unsafe { thread.assume_init() };

        return Thread::from_value(thread);
    }
}

impl Thread {
    pub fn from_ptr(thread: *mut clib_ncvm::ncvm_thread) -> Thread {
        Thread {
            c_thread: PtrOrValue::Ptr(thread),
        }
    }

    pub fn from_value(thread: clib_ncvm::ncvm_thread) -> Thread {
        Thread {
            c_thread: PtrOrValue::Value(thread),
        }
    }

    pub fn run(&mut self) {
        unsafe {
            match self.c_thread {
                PtrOrValue::Ptr(ptr) => clib_ncvm::ncvm_execute_thread(ptr),
                PtrOrValue::Value(mut th) => {
                    clib_ncvm::ncvm_execute_thread(&mut th as *mut clib_ncvm::ncvm_thread)
                }
            };
            //clib_ncvm::ncvm_run_thread(self.c_thread.as_mut_ptr());
        }
    }

    pub fn get_u32_reg(&self, index: usize) -> u32 {
        unsafe {
            //clib_ncvm::ncvm_get_u32_reg(self.c_thread.as_mut_ptr(), index)
            0
        }
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        unsafe {
            clib_ncvm::ncvm_free(&mut self.c_vm as *mut clib_ncvm::ncvm);
        }
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe {
            match self.c_thread {
                PtrOrValue::Ptr(ptr) => {}
                PtrOrValue::Value(mut th) => {
                    clib_ncvm::ncvm_thread_free(&mut th as *mut clib_ncvm::ncvm_thread)
                }
            };
        }
    }
}
