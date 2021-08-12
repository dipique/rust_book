mod statistics;
mod pig_latin;

fn main() {
    // let nums = vec![14.3, 5.7, 9.1, 3.3];
    let nums = vec![1.0, 2.0, 2.0, 3.0, 4.0, 5.0, 4.0, 4.0, 6.0, 8.0];
    println!("Len is: {}", nums.len());
    println!("Sum is: {}", statistics::sum(&nums));
    println!("Mean is: {}", statistics::mean(&nums));
    println!("Median is: {}", statistics::median(&nums));
    println!("Mode is: {}", statistics::mode(&nums));

    // make sure we aren't dropping our vector
    println!("Vec: {:?}", nums);

    println!("'brought' in pig latin: '{}'", pig_latin::to_pig_latin("brought"));
    println!("'apple' in pig latin: '{}'"  , pig_latin::to_pig_latin("apple"));
    println!("'a' in pig latin: '{}'"      , pig_latin::to_pig_latin("a"));
    println!("'jump' in pig latin: '{}'"   , pig_latin::to_pig_latin("jump"));

}
