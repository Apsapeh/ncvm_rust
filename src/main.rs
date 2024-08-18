#![no_main]

use std::collections::HashMap;


use ncvm::NativeFunction;

#[ncvm::native_function]
pub fn print_pi(th: ncvm::Thread) {
    println!("{}", th.get_f64_reg(1));
    //std::io::stdout().write("euo".as_bytes()).unwrap();
}

#[no_mangle]
fn main() {
    //let dv = std::fs::read("perf_test.ncvm").unwrap();
    let mut hasmp_libs: HashMap<String, NativeFunction> = HashMap::new();
    //hasmp_libs["pi"] = print_pi as unsafe extern "C" fn(*mut ::ncvm::clib_ncvm::ncvm_thread);
    hasmp_libs.insert("print_pi".to_string(), print_pi as NativeFunction);
    hasmp_libs.insert("print_pi_2".to_string(), print_pi as NativeFunction);
    let native_fns_map = Some(ncvm::NativeFunctions::Functions(hasmp_libs));
    let native_fns_files = Some(ncvm::NativeFunctions::Files(vec![
        "examples/libs/mac_arm64.dylib".to_string(),
        //"/home/ghost/Desktop/Rust/Projects/ncvm_rust/liblib1.so".to_string(),
    ]));
    //let mut vm = ncvm::VM::from_data(dv, native_fns_map).unwrap();
    let mut vm = ncvm::VM::from_file("perf_test.ncvm", native_fns_map).unwrap();

    //println!("{}", vm.c_vm.inst_count);

    let thread = vm.create_thread(0, None, None);
    thread.run();
    //println!("R: {}", thread.get_f64_reg(1));
    //thread.run();
    let thread = vm.create_thread(0, None, None);
    //thread.run();

    //ncvm_execute_thread(&mut thread.c_thread);
}
