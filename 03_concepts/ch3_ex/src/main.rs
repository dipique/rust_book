use std::io;

fn main() {
    let tf = 12f32;
    println!("In f: {} - in c: {}", tf, fahrenheit_to_celsius(tf));

    let tc = 44f32;
    println!("In c: {} - in f: {}", tc, celsius_to_fahrenheit(tc));

    let fib_num = generate_nth_fibinacci_num(15);
    println!("Fib num: {}", fib_num);
    //inter_ctof();
}

fn fahrenheit_to_celsius(temp_in_f: f32) -> f32 {
    (temp_in_f - 32f32) / 1.8
}

fn celsius_to_fahrenheit(temp_in_c: f32) -> f32 {
    (temp_in_c * 1.8) + 32f32
}

fn inter_ctof() {
    println!("Please enter a temperature in celsius");
    let mut in_tmp = String::new();
    io::stdin()
        .read_line(&mut in_tmp)
        .expect("Failed to read input");
    
    let in_tmp: f32 = match in_tmp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a number");
            return;
        },
    };

    println!("In c: {} - in f: {}", in_tmp, celsius_to_fahrenheit(in_tmp));
}

fn generate_nth_fibinacci_num(n: i32) -> i32 {
    let mut prev: i32 = 0;
    let mut cur: i32 = 0;
    let mut next: i32 = 1;
    for i in 0..n {
        println!("i: {}, f: {}", i, cur);
        prev = cur;
        cur = next;

        next = get_next_fib_num(prev, cur);
    }
    return cur;
}

fn get_next_fib_num(last_fib_num: i32, current_fib_num: i32) -> i32 {
    last_fib_num + current_fib_num
}