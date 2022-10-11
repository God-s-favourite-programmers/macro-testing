use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};
use std::fs;

#[proc_macro]
pub fn import(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let dir_path = input.value();
    let dir = match fs::read_dir(dir_path) {
        Ok(dir) => dir,
        Err(e) => panic!("{}", e)
    };

    let mut imports = String::new();
    for file in dir {
        match file {
            Ok(file) => {
                let os_name = file.file_name();
                let name = os_name.to_str();
                match name {
                    Some(name) => {
                        if name == "mod.rs" {
                            continue;
                        }
                        let sanitized = name.split(".").next().unwrap();
                        imports.push_str(&format!("pub mod {};", sanitized));
                    },
                    None => panic!("Invalid file name: {:?}", os_name)
                }
            },
            Err(e) => panic!("Error reading file: {}", e)
        }
    }

    match imports.parse() {
        Ok(tok_stream) => tok_stream,
        Err(e) => panic!("Error parsing: {}", e)
    }
}


fn parse_inputs(input: TokenStream) -> (Option<String>, Option<String>) {
    let mut past_sep = false;
    let mut dir_name = None;
    let mut action_name = None;
    for tok in input.into_iter() {
        if let proc_macro::TokenTree::Literal(lit) = tok {
            if past_sep {
                action_name = Some(lit.to_string());
            } else {
                dir_name = Some(lit.to_string());
            }
        } else if let proc_macro::TokenTree::Punct(_) = tok {
            past_sep = true;
        }
    }
    (dir_name, action_name)
}

#[proc_macro]
pub fn use_actions(input: TokenStream) -> TokenStream {
    let (dir_name, action_name) = parse_inputs(input);
    
}