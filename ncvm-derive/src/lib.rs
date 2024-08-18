extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn native_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_vis = &input_fn.vis;
    let fn_params = match input_fn.sig.inputs.first() {
        Some(param) => param,
        None => panic!("Expected a function parameter 'ncvm::Thread'"),
    };
    let fn_param_pat = match fn_params {
        syn::FnArg::Typed(pat_type) => pat_type,
        _ => panic!("Expected a function parameter"),
    };
    let fn_param_name = fn_param_pat.pat.as_ref();
    let fn_param_type = fn_param_pat.ty.as_ref();
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;

    /*if fn_param_type.to_token_stream().to_string() != "Thread" {
        panic!("Expected a function parameter '&mut Thread'");
    }*/

    let result = quote! {
        #fn_vis unsafe extern "C" fn #fn_name(unsafe_thread: *mut ::ncvm::clib_ncvm::ncvm_thread) {
            //let #fn_param_name = ncvm::Thread {};
            let #fn_param_name : #fn_param_type = #fn_param_type::from_ptr(unsafe_thread);
            //println!("Entering {}", stringify!(#fn_param_name));
            #fn_body
            //println!("Exiting {}", stringify!(#fn_param_type));
        }
    };

    result.into()
}
