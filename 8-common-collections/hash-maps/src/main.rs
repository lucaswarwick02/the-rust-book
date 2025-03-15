use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    let mut scores: HashMap<String, usize> = HashMap::new();

    // We can add using insert
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Can get using the value
    let team_name = String::from("Blue");

    // Note: `get` returns None if the key doesn't exist, therefore we can cast with `unwrap_or`
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{field_name}: {field_value}"); // Error! Borrow of moved value

    // If we insert using the same key, it overwrites it
    let mut scores: HashMap<String, usize> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");  // Output: `{"Blue": 25}`

    let mut scores: HashMap<String, usize> = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);  // Adds `Yellow: 50`
    scores.entry(String::from("Blue")).or_insert(50);  // Does not add, keeps `Blue: 10`

    println!("{scores:?}");  // Output: `{"Blue": 10, "Yellow": 50}`

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);  // Set/get 0 if not there
        *count += 1;  // Dereference with `*` and update
    }

    println!("{map:?}");  // Output: `{"wonderful": 1, "hello": 1, "world": 2}`
}
