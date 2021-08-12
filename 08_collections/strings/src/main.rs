fn main() {
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();

    // grow a string
    let mut s = String::from("foo");
    s.push(' '); // add single character
    s.push_str("bar");

    // concatenate
    // s = data + s; // cannot do this, because you can't add a String to a string slice
    s = s + data;    // but you can add a string slice to a string *oh god*
    let s2 = String::from("value: ");
    let s3 = s2 + &s; // uses the add method, which is ~ fn add(self, s: &str) -> String {}
    // println!("s2 value: {}", s2); // invalid: s2 has been moved now and can no longer be used
    println!("s3 value: {}", s3);

    // when we add String + &String, it doesn't match signature (String, &str)
    // But it still compiles because &String can be coerced to &str; this is called deref coercion

    let t = String::from("first ");
    let u = String::from("second");
    let v = t + &u;
    //println!("t'{}' u'{}' v'{}'", t, u, v); // we cannot use t anymore
    let w = u.to_string() + &v;
    println!("u'{}' v'{}' w'{}'", u, v, w); // but here, we have all three by copying u
    
    // or we can use format! macro
    let x = format!("u'{}' v'{}' w'{}'", u, v, w); // does not take ownership

    // let c = x[1]; // you can't index Strings in Rust; this is because a UTF8 string
    // contains elements that don't have a fixed length. The indexing operation is
    // expected to take constant time O(1), but this can't be guaranteed when returning
    // a char index may require walking the entire string.

    // More accurately, Strings are stored as u8 values which ARE fixed length; but each
    // u8 value does not (necessarily) correspond to a single character.

    // however, you can create a String slice using a range
    let y = &x[2..4];

    // iterate over chars
    for c in y.chars() {
        println!("{}", c);
    } // not the same as iterating over grapheme clusters, which is more complex

    // iterate over bytes
    for b in y.bytes() {
        println!("{}", b);
    }
}
