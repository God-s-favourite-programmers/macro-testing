use proc_macro::{TokenStream, Ident, Span, TokenTree};


#[proc_macro]
pub fn use_actions(input: TokenStream) -> TokenStream {
    // Find dir to populate output with
    let dir_path = "";
    for tok in input.into_iter() {
        if  
    }



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