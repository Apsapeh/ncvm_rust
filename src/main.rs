use std::{
    collections::HashMap, os::raw::c_void, ptr::{null, null_mut}
};

use ncvm::{clib_ncvm::{
    ncvm_create_thread, ncvm_default_get_lib_function, ncvm_default_lib_loader, ncvm_default_lib_loader_init, ncvm_execute_thread, ncvm_lib_function, ncvm_loadBytecodeStream, ncvm_thread, ThreadSettings
}, NativeFunction};

#[ncvm::native_function]
pub fn print_pi(th: ncvm::Thread) {
    unsafe {
        println!("{}", 3.14);
    }
}

static mut last: usize = 0;
static mut data_vec: Vec<u8> = Vec::new();

unsafe extern "C" fn get_next_n_bytes(
    n: ::std::os::raw::c_ulong,
    mut data_p: *mut ::std::os::raw::c_void,
) -> *const u8 {
    if last == 0 {
        unsafe {
            data_vec = std::fs::read("perf_test.ncvm").unwrap();
        };
    }

    last += n as usize;
    if last > data_vec.len() {
        null()
    } else {
        data_vec.as_ptr().add(last - n as usize) as *const u8
    }
}

fn main() {
    let libs = vec!["/home/ghost/Desktop/Rust/Projects/ncvm_rust/liblib1.so\0"];

    let mut lib_loader = unsafe {
        let l = libs.as_ptr() as *mut *const ::std::os::raw::c_char;
        ncvm_default_lib_loader_init(l, libs.len() as ::std::os::raw::c_ulong)
    };

    unsafe {
        let dv = std::fs::read("perf_test.ncvm").unwrap();
        let mut hasmp_libs: HashMap<String, NativeFunction> = HashMap::new();
        //hasmp_libs["pi"] = print_pi as unsafe extern "C" fn(*mut ::ncvm::clib_ncvm::ncvm_thread);
        hasmp_libs.insert("print_pi".to_string(), print_pi as NativeFunction);
        hasmp_libs.insert("print_pi_2".to_string(), print_pi as NativeFunction);
        println!("hasmp_libs: {:?}", hasmp_libs);
        let native_fns_map = Some(ncvm::NativeFunctions::Functions(hasmp_libs));
        let native_fns_files = Some(ncvm::NativeFunctions::Files(vec!["/home/ghost/Desktop/Rust/Projects/ncvm_rust/liblib1.so".to_string()]));
        let mut vm = ncvm::VM::from_data(dv, native_fns_map).unwrap();
        /*let settings = ThreadSettings {
            u32_reg_size: 8,
            u64_reg_size: 8,
            f32_reg_size: 8,
            f64_reg_size: 8,
            stack_size: 1024 * 1024 * 1, // 1 MiB
            call_stack_size: 1024 * 1024 * 1,
        };
        let mut vm: ncvm = ncvm {
            inst_p: null_mut(),
            static_mem_p: null_mut(),
            inst_count: 0,
            static_mem_size: 0,
            main_thread_settings: settings,
            lib_functions: null_mut(),
        };
        let ptr_vm = &mut vm as *mut ncvm;
        let gn = Some(
            get_next_n_bytes
                as unsafe extern "C" fn(
                    ::std::os::raw::c_ulong,
                    *mut ::std::os::raw::c_void,
                ) -> *const u8,
        );

        let gl = Some(
            ncvm_default_get_lib_function
                as unsafe extern "C" fn(
                    *const ::std::os::raw::c_char,
                    *mut ::std::os::raw::c_void,
                ) -> ncvm_lib_function,
        );

        let r = ncvm_loadBytecodeStream(
            ptr_vm,
            gn,
            null_mut(),
            gl,
            &mut lib_loader as *mut ncvm_default_lib_loader as *mut ::std::os::raw::c_void,
        );*/
        //println!("R: {}", r);

        println!("{}", vm.c_vm.inst_count);

        /*let mut thread = ncvm_thread {
            current_instr_p: null(),
            stack_p: null_mut(),
            call_stack_p: null_mut(),
            vm: null_mut(),
            u32_registers: null_mut(),
            u64_registers: null_mut(),
            f32_registers: null_mut(),
            f64_registers: null_mut(),
        };

        ncvm_create_thread(
            &mut thread as *mut ncvm_thread,
            &mut vm.c_vm as *mut ncvm::clib_ncvm::ncvm,
            vm.c_vm.inst_p,
            null_mut(),
            0,
            vm.c_vm.main_thread_settings,
        );*/
        let mut thread = vm.create_thread(0, None, None);
        thread.run();
        thread.run();

        //ncvm_execute_thread(&mut thread.c_thread);
    }
}
