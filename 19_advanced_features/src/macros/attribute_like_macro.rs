// e.g. #[route(GET, ",")]

// the signature contains a TokenStream for both the attribute contents and the fn itself

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}