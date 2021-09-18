pub mod token;
pub mod transform;
//
// use proc_macro::TokenStream;
//
// #[proc_macro]
// pub fn qas(path: TokenStream) -> TokenStream {
//     let src = path.to_string();
//     assert!(src[..1].chars().next().unwrap() == '"' && src[src.len() - 1..].chars().next().unwrap() == '"');
//     let path = &src[1..src.len() - 1];
//     transform::start(token::parse(std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{:?}: {}", e, path)).chars()), transform::Format::detect(path)).parse().unwrap()
// }
