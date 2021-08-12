fn main() {
    let mut u = User {
        username: String::from("dipique"), // in this case we use String instead of &str so that the
        email: String::from("dipique@gmail.com"), // struct will own all of its data
        sign_in_count: 0,
        active: true
    };

    println!("Username is: {}", u.username);
    
    u.username = String::from("dkaschel");

    println!("Never mind, it is: {}", u.username);

    // struct update syntax
    let user2 = User {
        username: String::from("dipique"),
        email: String::from("dipique@outlook.com"),
        ..u // uses the values from u other than those specified
    }; // note that unlike js, username and email are not overwritten
       // even though u has fields with the same name
    
    // tuple-like way of defining structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    // a function that requires a Color cannot take a Point as an arg
    // even though it has the same type parameters

    // Tuple structs behave much like tuples and can be destructures
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
