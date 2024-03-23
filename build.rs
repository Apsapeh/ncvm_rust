fn main() {
    // Compile library 
    cc::Build::new()
        .file("libs/ncvm/src/ncvm.c")
        .include("libs/ncvm/include")
        .include("libs/extc/include")
        .opt_level(3)
        .debug(false)
        .warnings(false)
        .compile("c-ncvm-static");


    let bindings = bindgen::Builder::default()
        .header("libs/ncvm/include/ncvm.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let mut result = move_const_to_enum(
        &bindings.to_string(), "Opcode", "OPCODE_"
    );
    result = move_const_to_enum(
        &result, "Register", "REGISTER_"
    );
    result.insert_str(0, "#![allow(warnings)]\n");
    //let result = bindings.to_string();

    std::fs::write("src/clib_ncvm.rs", result).unwrap();
}

fn move_const_to_enum(source: &String, enum_name: &str, const_keyword: &str) -> String {
    let mut result: String = String::with_capacity(source.len());
    let mut str_enum: String = String::from(format!("pub enum {} {{\n", enum_name));
    let mut before_enum = String::new();
    for line in source.split("\n") {
        let lw = line.split_whitespace().collect::<Vec<&str>>();
        
        if lw.len() == 6 && lw[0] == "pub" &&
           lw[1] == "const" && lw[2].starts_with(const_keyword) {
            str_enum.push_str(
                format!(
                    "\t{} = {}\n",
                    lw[2].replace(const_keyword, "").replace(":", ""),
                    lw[5].replace(";", ",")
                ).as_str()
            );
        }
        else if lw.len() >= 3 && lw[0] == "pub" &&
        lw[1] == "type" && format!("{}_", lw[2]) == const_keyword {
            before_enum = format!(
                "{}\n#[repr(u8)]\n",
                line.replace(const_keyword, "")
            );
            
            /*after_enum = format!(
                "use {}::{};\n", mod_name,
                const_keyword.chars().take(const_keyword.len()-1).collect::<String>()
            )*/
        }
        else {
            result.push_str(line);
            result.push('\n');
        }
    }
    str_enum.push_str("}\n");
    //str_enum.push_str(&after_enum);
    str_enum.insert_str(0, &before_enum);
    result.insert_str(0, &str_enum);
    result
}