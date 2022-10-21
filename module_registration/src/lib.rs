use std::{fs::{self, ReadDir}};

use proc_macro::{TokenStream};
use syn::{parse_macro_input, LitStr, parse::Parse};

fn get_file_names(dir: ReadDir) -> Vec<String> {
    let mut names = Vec::new();
    for file in dir {
        match file {
            Ok(file) => {
                let os_name = file.file_name();
                let name = os_name.to_str();
                match name {
                    Some(name) => {
                        if name == "mod.rs" || !name.ends_with(".rs"){
                            continue;
                        }
                        let sanitized = name.split(".").next().unwrap();
                        names.push(sanitized.to_string());
                    },
                    None => panic!("Invalid file name: {:?}", os_name)
                }
            },
            Err(e) => panic!("Error reading file: {}", e)
        }
    }
    names
}


#[proc_macro]
pub fn import(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let dir_path = input.value();
    let dir = match fs::read_dir(dir_path) {
        Ok(dir) => dir,
        Err(e) => panic!("{}", e)
    };

    let imports = get_file_names(dir).iter().map(|name| format!("pub mod {};", name)).collect::<Vec<String>>().join("");


    match imports.parse() {
        Ok(tok_stream) => tok_stream,
        Err(e) => panic!("Error parsing: {}", e)
    }
}

struct InvocationTarget {
    directory: LitStr,
    function_name: LitStr
}

impl Parse for InvocationTarget {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let directory: LitStr = input.parse()?;
        let function_name: LitStr = input.parse()?;
        Ok(InvocationTarget { directory, function_name })
    }
}

#[proc_macro]
pub fn invoke(input: TokenStream) -> TokenStream {
    let InvocationTarget { directory, function_name } = parse_macro_input!(input as InvocationTarget);
    
    let dir = match fs::read_dir(directory.value()) {
        Ok(dir) => dir,
        Err(e) => panic!("{}", e)
    };

    let names = get_file_names(dir);
    let mut output = String::new();
    for name in names {
        output.push_str(&format!("\"{name}\" => {}::{}(),\n", directory.value().split("/").map(|s| s.to_string()).collect::<Vec<String>>().join("::"), function_name.value()))
    }

    match output.parse() {
        Ok(tok_stream) => tok_stream,
        Err(e) => panic!("Error parsing: {}", e)
    }
}


/*
#[proc_macro]
pub fn use_actions(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = input.into_iter().collect();

    // Find dir to populate output with
    let mut dir_path = String::new();
    for tok in tokens.iter() {
        match tok {
            TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
                // The stuff inside here should be the path to the directory we are interested in
                for path_part in group.stream() {
                    match path_part {
                        TokenTree::Ident(ident) => dir_path.push_str(&ident.to_string()),
                        _ => {dir_path.push('/')}
                    }
                }
            }
            _ => {}
        }
    }

    let mut files = Vec::new();
    let dir = fs::read_dir(dir_path).unwrap();
    for file in dir {
        let file = file.unwrap();
        let name = file.file_name();
        let file_type = file.file_type().unwrap();
        
        if file_type.is_file() && name != "mod.rs" && name.to_str().to_owned().unwrap().contains(".rs") {
            files.push(file.file_name());
        }
    }

    // UNder construction Henrik

    let mut output = Vec::new();
    for tok in input.into_iter() {
        match tok {
            TokenTree::Punct(punct) if punct == '*' => {output.push(TokenTree::Ident(Ident::new("name_from_macro", Span::call_site())));}
            TokenTree::Punct(_) => {output.push(tok)}
            _ => {output.push(tok);}
        };
    }
    let stream = TokenStream::from_iter(output.into_iter());
    stream
}
*/