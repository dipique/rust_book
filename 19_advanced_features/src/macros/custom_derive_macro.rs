extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap(); // creates syntax tree we can manipulate
    impl_hello_macro(&ast);
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident; // e.g. Pancakes
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}}!", stringify!(#name));
                // stringify is built into rust and turns a rust expression (e.g. 1 + 2)
                // into a string such as "1 + 2"
            }
        }
    };
    gen.into()
}