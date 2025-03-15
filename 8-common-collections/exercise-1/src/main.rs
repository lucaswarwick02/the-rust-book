use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct Mode {
    value: i8,
    count: i8,
}

fn main() {
    // Median = 5
    // Mode = 1 and 5!
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];

    let modes = mode(&numbers);
    println!("The mode of numbers is: {modes:?}");

    let median = median(&numbers);
    println!("The median of numbers is: {median}");

    println!("Numbers are: {numbers:?}");
}

/// Since `mode` can have multiple values, we want to return a 
/// list of Mode's, containing both the value and the count.
fn mode(v: &Vec<i8>) -> Vec<Mode> {
    let mut counter: HashMap<i8, i8> = HashMap::new();

    // Calculate the count of each item
    for &n in v {
        let count = counter.entry(n).or_insert(0);
        *count += 1;
    }

    // Grab maximum count
    let max_count = *counter.values().max().unwrap_or(&-1);

    let mut modes: Vec<Mode> = Vec::new();

    // Grab the values which have the highest count
    for (&key, &value) in &counter {
        if value == max_count {
            modes.push(Mode{
                value: key,
                count: value
            });
        }
    }

    return modes;
}

fn median(v: &Vec<i8>) -> f32 {
    let mut sorted = v.clone();
    sorted.sort();

    if v.len() % 2 == 0 {
        // Even amount, get middle 2 and average
        ((sorted[(v.len() / 2) - 1] + sorted[v.len() / 2]) as f32) / 2.0
    }
    else {
        // Odd, get the middle
        sorted[v.len() / 2] as f32
    }
}