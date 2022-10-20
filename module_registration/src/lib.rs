use std::fs;

use proc_macro::{TokenStream, Ident, Span, TokenTree, Delimiter};


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
