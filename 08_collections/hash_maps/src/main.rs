use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10); // value type established here
    scores.insert(String::from("Blue"), 50); // all keys must be same type, and all values

    // creating a HashMap from keys and values
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip( // zip is kind of creating a collection of tuples
            initial_scores.into_iter()
        ).collect();
    scores.insert(String::from("Orange"), 90);

    // accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // result is Some(&10);

    // iterating over HashMap
    for (key, val) in &scores {
        println!("key {}, val {}", key, val);
    }

    // inserting into a hashmap will overwrite if the value already exists
    scores.insert(team_name.to_string(), 35);
    println!("key {}, val {}", &team_name, scores.get(&team_name).unwrap()); // I think this is an unsafe unwrap

    // insert only if thre isn't an existing value
    scores.entry(team_name.to_string()).or_insert(90); // won't insert because blue exists
    scores.entry(String::from("Red")).or_insert(90); // won't insert because blue exists
    for (key, val) in &scores {
        println!("key {}, val {}", key, val);
    }

    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // mututable reference to value
        *count += 1;
    }

    println!("{:?}", map);
}
