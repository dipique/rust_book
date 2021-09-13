// this is how the vec! macro works (simplified)

#[macro_export] // should be made available when crate is brought into scope
macro_rules! vec {
    // this is a pattern to match; this one only has one arm,
    // but more complex macros can have multiple arms
    ( $( $x:expr ),* ) => { // expr matches any expression; * means match 0 or more times
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    };
}

// this type of macro is more likely to be used than written, apparently