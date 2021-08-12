fn main() {
    let mut v: Vec<i32> = Vec::new();
    let vi = vec![1,2,3]; // type inferred from values

    // vectors are more like a list than an array because you can change its length
    // (but only if it is mutable)
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);

    // to reference an element, we have to borrow it
    let third = &v[2]; // &i32
    println!("Third value is {}", third);

    // we can use .get to get an Option<&t>
    match v.get(4) {
        Some(fifth) => println!("Fifth value is: {}", fifth),
        None => println!("There is no fifth element")
    }

    iterating_vec();
}

enum SpreadsheetCell {
    Int(i32),
    Text(String),
    Float(f64)
}

fn store_diff_types_in_vec() {
    let row = vec![ // these elements all have the same type (SpreadsheetCell)
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
}

fn iterating_vec() {
    // iterate immutably
    let v = vec![100, 32, 57];
    for i in &v { 
        println!("Value of element is {}", i);
    }

    // iterate mutably
    let mut v = vec![100,32,57];
    for i in &mut v { // i is &mut i32
        *i += 50; // use dereference operator to get to the value in i
    }
}

fn borrow_checking() {
    let mut v = vec![1,2,3,4,5];
    let first = &v[0]; // immutable borrow
    //v.push(6); // mutating array renders immutable borrow invalid
    println!("The first element is {}", first); // using the immutable borrow creates compilation err

    // this creates an error because adding to the array involves (or may involve) copying the array
    // to a new location, so a reference to an element will prevent mutating the array
}

fn induce_panic() {
    let v = vec![1,2,3,4,5];
    let does_not_exist = &v[100]; // panic will occur here
    let does_not_exist = v.get(100);
}