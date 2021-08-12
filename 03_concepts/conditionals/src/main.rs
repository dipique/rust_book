fn main() {
    basic_if();
    ternary();
    loop_without_return();
    return_from_loop();
    is_num(2);
    while_cond_loop();
    for_loop();
    range_generator();
}

fn basic_if() {
    let x = 5;
    let y = 5;
    if x > y {
        println!("x ({}) is greater than y({})", x, y);
    } else {
        println!("x({}) is not greater than y({})", x, y);
    }
}
fn loop_without_return() {
    let mut x = 1;
    loop {
        if x > 5 {
            break;
        }
        x+=1;
    }
    println!("The loop counter variable value is: {}", x);
}

fn return_from_loop() {
    let mut x = 1;
    let val = loop {
        if  x > 5 {
            break x * 2;
        }
        x += 2;
    }; // note that this requires a semicolon at the end because it is in an expression
    println!("The loop return value was: {}", val);
}

fn ternary() {
    let x = 5;
    let y = if x == 5 { "yes" } else { "no" };
    println!("The value of y is {}", y);
}

fn is_num(number: u32) { 
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        _ => println!("Something else")
    }
    
}

fn while_cond_loop() {
    let mut number = 11 ;
    while number > 0 {
        println!("Loop var is: {}", number);
        number -= 2;
    }
}

fn for_loop() {
    let a = [10, 20, 30, 40];
    for x in a.iter() {
        println!("Value of x is {}", x);
    }
    println!("And now in reverse...");
    for x in a.iter().rev() {
        println!("Value of x is {}", x);
    }
}

fn range_generator() {
    // this prints out 1..9; ending is non-inclusive
    println!("Create the range 1..10 which prints 1-9 as it is non-inclusive");
    for x in 1..10 {
        println!("Value of x is {}", x);
    }

    //if we want the ending to be inclusive:
    println!("Same thing, but with an inclusive range (1..=10)");
    for x in 1..=10 {
        println!("Value of x is {}", x);
    }
}