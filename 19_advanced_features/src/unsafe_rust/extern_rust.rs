// damn that's actually really easy. It uses the application binary interface (ABI);
// the most common one is "C"
extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn call_c_function() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// calling rust from other languages
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}