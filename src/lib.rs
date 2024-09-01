use std::{collections::HashMap, ptr::null_mut};

pub use ncvm_derive::native_function;

pub mod clib_ncvm;

pub type NativeFunction = unsafe extern "C" fn(*mut clib_ncvm::ncvm_thread);

#[repr(u8)]
#[derive(Debug)]
pub enum Error {
    U32Not32Bit = clib_ncvm::NCVM_U32_NOT_32_BIT as u8,
    U64Not64Bit = clib_ncvm::NCVM_U64_NOT_64_BIT as u8,
    IsBigEndian = clib_ncvm::NCVM_IS_BIG_ENDIAN as u8,
    StackAllocError = clib_ncvm::NCVM_STACK_ALLOC_ERROR as u8,
    IncompatibleVersion = clib_ncvm::NCVM_INCOMPATIBLE_VERSION as u8,
    NativeFunctionNotFound = clib_ncvm::NCVM_LIB_FUNCTION_NOT_FOUND as u8,
    BytecodeReadError = clib_ncvm::NCVM_BYTECODE_READ_ERROR as u8,
    FunctionNotFound = clib_ncvm::NCVM_FUNCTION_NOT_FOUND as u8,
    StackOverflow = clib_ncvm::NCVM_STACK_OVERFLOW as u8,
    StackUnderflow = clib_ncvm::NCVM_STACK_UNDERFLOW as u8,
    CallStackOverflow = clib_ncvm::NCVM_CALL_STACK_OVERFLOW as u8,
}

enum PtrOrValue<T> {
    Ptr(*mut T),
    Value(T),
}

pub enum NativeFunctions {
    Files(Vec<String>),
    Functions(HashMap<String, NativeFunction>),
    //CustomGetter(fn(name: &str, data: &dyn std::any::Any) -> NativeFunction),
}

pub struct VM {
    pub c_vm: clib_ncvm::ncvm,
    default_lib_loader: Option<clib_ncvm::ncvm_default_lib_loader>,
}

pub struct Thread {
    c_thread: PtrOrValue<clib_ncvm::ncvm_thread>,
}

struct DataPtrWrap {
    ptr: *const u8,
}

type LibGetterFunction = unsafe extern "C" fn(
    *const ::std::os::raw::c_char,
    *mut ::std::os::raw::c_void,
) -> clib_ncvm::ncvm_lib_function;

impl VM {
    pub fn from_file(path: &str, native_fns: Option<NativeFunctions>) -> Result<VM, Error> {
        let bytecode = std::fs::read(path).unwrap();
        VM::from_data(bytecode, native_fns)
    }

    pub fn from_data(bytecode: Vec<u8>, native_fns: Option<NativeFunctions>) -> Result<VM, Error> {
        unsafe {
            //a(null_mut());
            let mut data_ptr = DataPtrWrap {
                ptr: bytecode.as_ptr(),
            };

            let mut vm = std::mem::MaybeUninit::uninit();
            let gn = Some(
                VM::get_next_n_bytes
                    as unsafe extern "C" fn(
                        ::std::os::raw::c_ulong,
                        *mut ::std::os::raw::c_void,
                    ) -> *const u8,
            );

            let mut is_default_lib_loader = false;
            let (lib_getter, lib_getter_data) = match native_fns {
                Some(NativeFunctions::Files(files)) => {
                    let mut cstr_libs = files
                        .into_iter()
                        .map(|s| std::ffi::CString::new(s).unwrap().into_raw() as *const i8)
                        .collect::<Vec<_>>();
                    let lib_loader = clib_ncvm::ncvm_default_lib_loader_init(
                        cstr_libs.as_mut_ptr(),
                        cstr_libs.len() as ::std::os::raw::c_ulong,
                    );

                    is_default_lib_loader = true;
                    (
                        Some(clib_ncvm::ncvm_default_get_lib_function as LibGetterFunction),
                        Box::into_raw(Box::new(lib_loader)) as *mut ::std::os::raw::c_void,
                    )
                }
                Some(NativeFunctions::Functions(functions)) => {
                    //println!("functions: {:?}", functions);
                    (
                        Some(VM::rust_lib_getter as LibGetterFunction),
                        Box::into_raw(Box::new(functions)) as *mut ::std::os::raw::c_void,
                    )
                }
                None => (Some(VM::empty_lib_getter as LibGetterFunction), null_mut()),
            };

            //println!("data: {:?}", lib_getter_data as *mut HashMap<String, NativeFunction> as &mut HashMap<String, NativeFunction>);

            let r = clib_ncvm::ncvm_loadBytecodeStream(
                vm.as_mut_ptr(),
                gn,
                &mut data_ptr as *mut DataPtrWrap as *mut ::std::os::raw::c_void,
                lib_getter,
                //&mut lib_loader as *mut clib_ncvm::ncvm_default_lib_loader as *mut ::std::os::raw::c_void,
                lib_getter_data,
            );

            let mut default_lib_loader_result = None;
            if is_default_lib_loader {
                default_lib_loader_result =
                    Some(*(lib_getter_data as *mut clib_ncvm::ncvm_default_lib_loader));
            }

            if r != clib_ncvm::NCVM_OK as u8 {
                return Err(Error::from_code(r).unwrap());
            }

            Ok(VM {
                c_vm: vm.assume_init(),
                default_lib_loader: default_lib_loader_result,
            })
        }
    }

    pub fn find_function_addr(&mut self, name: &str) -> Option<u64> {
        unsafe {
            let mut addr = std::mem::MaybeUninit::uninit();
            let name = std::ffi::CString::new(name).unwrap();
            let r = clib_ncvm::ncvm_find_function_addr(
                &mut self.c_vm,
                name.as_ptr(),
                addr.as_mut_ptr(),
            );

            if r == clib_ncvm::NCVM_OK as u8 {
                return Some(addr.assume_init());
            }
            None
        }
    }

    unsafe extern "C" fn get_next_n_bytes(
        n: ::std::os::raw::c_ulong,
        data_p: *mut ::std::os::raw::c_void,
    ) -> *const u8 {
        let data = &mut *(data_p as *mut DataPtrWrap);

        let return_ptr = data.ptr;
        data.ptr = data.ptr.add(n as usize);
        return_ptr
    }

    unsafe extern "C" fn empty_lib_getter(
        _: *const ::std::os::raw::c_char,
        _: *mut ::std::os::raw::c_void,
    ) -> clib_ncvm::ncvm_lib_function {
        None
    }

    unsafe extern "C" fn rust_lib_getter(
        name: *const ::std::os::raw::c_char,
        data_p: *mut ::std::os::raw::c_void,
    ) -> clib_ncvm::ncvm_lib_function {
        let data = &*(data_p as *const HashMap<String, NativeFunction>);
        let name = std::ffi::CStr::from_ptr(name).to_str().unwrap();

        match (*data).get(name) {
            Some(f) => Some(*f),
            None => None,
        }
    }
}

// Thread functions
impl VM {
    pub fn create_thread(
        &mut self,
        start_instruction_index: usize,
        ext_stack: Option<&mut Vec<u8>>,
        settings: Option<clib_ncvm::ThreadSettings>,
    ) -> Result<Thread, Error> {
        let mut thread = std::mem::MaybeUninit::uninit();
        //unsafe {
        let (ext_stack_ptr, ext_stack_len) = match ext_stack {
            Some(stack) => (stack.as_mut_ptr(), stack.len() as ::std::os::raw::c_ulong),
            None => (null_mut(), 0),
        };
        let settings = match settings {
            Some(settings) => settings,
            None => self.c_vm.main_thread_settings,
        };
        let r = unsafe {
            clib_ncvm::ncvm_create_thread(
                thread.as_mut_ptr(),
                &mut self.c_vm,
                self.c_vm.inst_p.add(start_instruction_index),
                ext_stack_ptr,
                ext_stack_len,
                settings,
            )
        };

        if r != clib_ncvm::NCVM_OK as u8 {
            return Err(Error::from_code(r).unwrap());
        }

        let thread = unsafe { thread.assume_init() };

        return Ok(Thread::from_value(thread));
    }

    pub fn create_function_thread(&mut self, name: &str) -> Result<Thread, Error> {
        let addr = match self.find_function_addr(name) {
            Some(addr) => addr,
            None => return Err(Error::FunctionNotFound),
        };

        self.create_thread(addr as usize, None, None)
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

    pub fn run(self) {
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
            match self.c_thread {
                PtrOrValue::Ptr(ptr) => (*ptr).u32_registers.add(index).read(),
                PtrOrValue::Value(th) => th.u32_registers.add(index).read(),
            }
        }
    }

    pub fn get_u64_reg(&self, index: usize) -> u64 {
        unsafe {
            match self.c_thread {
                PtrOrValue::Ptr(ptr) => (*ptr).u64_registers.add(index).read(),
                PtrOrValue::Value(th) => th.u64_registers.add(index).read(),
            }
        }
    }

    pub fn get_f32_reg(&self, index: usize) -> f32 {
        unsafe {
            match self.c_thread {
                PtrOrValue::Ptr(ptr) => (*ptr).f32_registers.add(index).read(),
                PtrOrValue::Value(th) => th.f32_registers.add(index).read(),
            }
        }
    }

    pub fn get_f64_reg(&self, index: usize) -> f64 {
        unsafe {
            //clib_ncvm::ncvm_get_f64_reg(self.c_thread.as_mut_ptr(), index)
            match self.c_thread {
                PtrOrValue::Ptr(ptr) => (*ptr).f64_registers.add(index).read(),
                PtrOrValue::Value(th) => th.f64_registers.add(index).read(),
            }
        }
    }
}

impl Error {
    pub fn from_code(code: u8) -> Option<Error> {
        match code as u32 {
            clib_ncvm::NCVM_U32_NOT_32_BIT => Some(Error::U32Not32Bit),
            clib_ncvm::NCVM_U64_NOT_64_BIT => Some(Error::U64Not64Bit),
            clib_ncvm::NCVM_IS_BIG_ENDIAN => Some(Error::IsBigEndian),
            clib_ncvm::NCVM_STACK_ALLOC_ERROR => Some(Error::StackAllocError),
            clib_ncvm::NCVM_INCOMPATIBLE_VERSION => Some(Error::IncompatibleVersion),
            clib_ncvm::NCVM_LIB_FUNCTION_NOT_FOUND => Some(Error::NativeFunctionNotFound),
            clib_ncvm::NCVM_BYTECODE_READ_ERROR => Some(Error::BytecodeReadError),
            clib_ncvm::NCVM_FUNCTION_NOT_FOUND => Some(Error::FunctionNotFound),
            clib_ncvm::NCVM_STACK_OVERFLOW => Some(Error::StackOverflow),
            clib_ncvm::NCVM_STACK_UNDERFLOW => Some(Error::StackUnderflow),
            clib_ncvm::NCVM_CALL_STACK_OVERFLOW => Some(Error::CallStackOverflow),
            _ => None,
        }
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        unsafe {
            if let Some(mut lib_loader) = self.default_lib_loader {
                clib_ncvm::ncvm_default_lib_function_loader_free(
                    &mut lib_loader as *mut clib_ncvm::ncvm_default_lib_loader,
                );
            }
            clib_ncvm::ncvm_free(&mut self.c_vm as *mut clib_ncvm::ncvm);
        }
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe {
            if let PtrOrValue::Value(mut th) = self.c_thread {
                clib_ncvm::ncvm_thread_free(&mut th as *mut clib_ncvm::ncvm_thread)
            }
        }
    }
}
