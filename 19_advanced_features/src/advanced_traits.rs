mod supertraits;
mod default_generic_types;
mod fully_qualified_syntax;
mod newtype_pattern;

// traits as interfaces (e.g. iterator) (no code, this is obvious)

pub fn run() {
    default_generic_types::run();
    supertraits::run();
    fully_qualified_syntax::run();
    newtype_pattern::run();
}