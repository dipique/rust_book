use std::slice;
mod extern_rust;

pub fn run() {
    create_raw_pointers_from_references();

    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (a,b) = my_split_at_mut(r, 3);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);

    extern_rust::call_c_function();
    static_vars();
    call_dangerous_function();
}

fn create_raw_pointers_from_references() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // note that we can do this safely, we just
    // can't de-reference them without unsafe

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

// unsafe function
unsafe fn dangerous() {}

fn call_dangerous_function() {
    unsafe {
        dangerous();
    }
}

fn my_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn do_not_call_this() {
    // we don't own this memory, so calling this function would break
    let address = 0x012345usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
}

// static variables

static HELLO_WORLD: &str = "Hello, world";
// uses 'static lifetime (no specific annotation required)

// unlike constants, static variables have a fixed location in memory
// static variables can also be mutable (but it is unsafe)

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn static_vars() {
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    // this is unsafe because in a multi-thread
    // environment, there would likely be data
    // races; but in a single-thread environment,
    // this won't cause any problems
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// unsafe is also used for unions which is primary used with C code