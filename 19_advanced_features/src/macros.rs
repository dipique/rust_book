// there are three types of procedural macros:
// - Custom #[derive] macros that specify code added with the derive attribute
// - Attribute-like macros that define custom attributes
// - Function-like macros that look like macros but operate on the argument tokens

// macros must be called into scope BEFORE use (unlike functions)

// procedural macros must reside in their own crate with a special crate type (that
// will hopefully change in the future)
//      proc-macro = true

// example of something we might attempt to replace with a procedural macro
pub trait HelloMacro {
    fn hello_macro();
}

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}