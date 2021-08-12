fn main() {
    fn_test();
    //fn_w_param(5);
    fn_set_to_body();
    println!("Using a return value to inc 2: {}", fn_with_ret_val(10));
}

fn fn_test() {
    println!("Hey, this is my fn test");
}

fn fn_w_param(i: i32) {
    println!("This is my age: {}", i);
}

fn fn_set_to_body() {
    let x = 5;
    let y = {
        let x = 3;
        x + 3 // no semicolon or this won't work
    };
    println!("This is my age: {}", y);
}

fn fn_with_ret_val(i: i32) -> i32 {
    i + 2
}

fn tuples() {
    let tup: (i32, f64, char) = (500, 6.3, 'z');
    println!("The value of x,y,z are {} {} {}", tup.0, tup.1, tup.2);

    // destructuring
    let (x, y, z) = tup;
    println!("The value of x,y,z are {} {} {}", x, y, z);
}

fn array() {
    let arr = [ 1, 5, 6 ];
    
    // destructuring
    let [x, y, z] = arr;
    println!("The value of x,y,z are {} {} {}", x, y, z);

    // type declaration for an array
    let typed_arr: [i32; 3] = [ 1, 2, 3];
    println!("The length of typedArr is {}", typed_arr.len());

    // string array
    let string_arr = [ "hello", "World" ]; // type is [&str; 2], so the contents are borrowed (and immutable?)

    // create an array with repeated value
    let repeated_val_arr = [ "howdy"; 4 ];
    println!("The values in repeated_val_arr are {} {} {} {}",
        repeated_val_arr[0], repeated_val_arr[1], repeated_val_arr[2], repeated_val_arr[3]
    );
}