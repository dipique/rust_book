fn main() {
    println!("{}", capitalize_first("hello"));
    let wv = capitalize_words_vector(&["hello", "world"]);
    for w in wv.iter() {
        println!("{}", w);
    }
}

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_ascii_uppercase().to_string() + c.as_str(),
    }
}

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|s| capitalize_first(s)).collect()
}