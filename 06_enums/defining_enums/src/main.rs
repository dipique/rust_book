fn main() {
    std_enum();

    // fancy new way of creating enum with associated value
    let home = IpAddrKindFancy::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKindFancy::V6(String::from("::1"));
    
    // each type can have different types and amounts of data
    let home = IpAddrKind::V4(127, 0, 0, 1);

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option enum
    let some_number = Some(5);
    let some_str = Some("a string"); // &str
    let absent_number: Option<i32> = None; // type specified because it can't be inferred

    // this code won't run
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // doesn't work because obvious reasons
}

// Option<T> is defined by the standard language as
// enum Option<T> {
//     Some(T),
//     None
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // named parameters (anonymous struct)
    Write(String),
    ChangeColor(i32, i32, i32), //allows trailing commas
}

// we can also write methods on enums
impl Message {
    fn call(&self) {
        // method body
    }
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

//////////////////////

enum IpAddrKindFancy {
    V4(String),
    V6(String)
}

////////////////////

fn std_enum() {
    let four = IpAddrKindStd::V4;
    let six = IpAddrKindStd::V6; // accessed like associated functions (i.e. static methods)

    let home = IpAddrStd {
        kind: IpAddrKindStd::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddrStd {
        kind: IpAddrKindStd::V6,
        address: String::from("::1")
    };
}

enum IpAddrKindStd {
    V4,
    V6
}

struct IpAddrStd {
    kind: IpAddrKindStd,
    address: String
}
