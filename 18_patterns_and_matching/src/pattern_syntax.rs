pub fn run() {
    match_named_variables();
    multiple_patterns();
    range_patterns();
    destructuring_structs();
    partial_destructure();
    destructuring_enums();
    at_bindings();
}

fn match_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn range_patterns() {
    let x = 5;

    match x {
        1..=5 => println!("one through 5"),
        _ => println!("something else"),
    }

    print_char_type('c');
    print_char_type('D');
    print_char_type('z');
    print_char_type('.');
    print_char_type('Y');
    print_char_type('_');
}

fn print_char_type(ltr: char) {
    println!("Char type input: {}", ltr);
    match ltr {
        'a'..='j' | 'A'..='J' => println!("early ASCII character"),
        'k'..='z' | 'K'..='Z' => println!("late ASCII character"),
        _ => println!("non-letter ascii (or non-ascii) character")
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructuring_structs() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b} = p;
    assert_eq!(0, a);
    assert_eq!(7, b); // okay, that's pretty useful

    // more terse way of doing the same thing:
    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // unnecessarily complicated example:
    let ((feet, inches), Point { x, y}) = ((3, 10), Point { x: 3, y: 56 });
}

fn partial_destructure() {
    let p = Point { x: 0, y: 7 };
    
    // this allows us to ignore unneeded values
    let Point {x, ..} = p;
    assert_eq!(0, x);

    // for tuple
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => println!("First: {}, last: {}", first, last),
    }

    match numbers {
        (_, second, ..) => println!("Second: {}", second),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn destructuring_enums() {
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure"),
        Message::Move { x, y} => println!("Move in x direction {} and y direction {}", x , y),
        Message::Write(text) => println!("Text message is: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!("Hue: {}, Saturation: {}, value: {}", h, s, v),
        // _ => (), // unneeded because the other arms are exhaustive
    }
}

fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x > 5 => println!("Int > 5: {}", x),
        Some(x) => println!("Any other int"),
        None => println!("Ain't no int at all"),
    }

    let x = 4;
    let y = false;

    match x {
        // IMPORTANT: the condition applies to ALL options (4, 5, 6), NOT just the last one
        4 | 5 | 6 if y => println!("Yes!"),
        _ => println!("No."),
    }
}

enum HelloMsg {
    Hello { id: i32 },
}

fn at_bindings() {
    let msg = HelloMsg::Hello { id: 5 };

    match msg {
        HelloMsg::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        HelloMsg::Hello { id: 10..=12 } => println!("Found an id in another range"),
        HelloMsg::Hello { id } => println!("Found some other id: {}", id),
    }
}