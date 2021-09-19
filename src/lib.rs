mod token;
mod transform;

use proc_macro::TokenStream;

#[proc_macro]
pub fn qas(path: TokenStream) -> TokenStream {
    let src = path.to_string();
    assert!(src[..1].chars().next().unwrap() == '"' && src[src.len() - 1..].chars().next().unwrap() == '"');
    let path = &src[1..src.len() - 1];
    let has_x11 = true;
    transform::start(path.to_string()).parse().unwrap()
}
