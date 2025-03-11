fn main() {
    let s = String::from("Hello, world!");

    // Grab the index of the first space
    let word_index = first_word_index(&s);

    let s = String::from("hello world");

    let hello = &s[0..5];  // Reference to indices 0 through 4
    let world = &s[6..11];  // Reference to indices 6 through 10


    let s = String::from("hello");

    // These are equivalent
    let slice = &s[0..2];
    let slice = &s[..2];

    let s = String::from("hello");

    let len = s.len();

    // These are also equivalent
    let slice = &s[3..len];
    let slice = &s[3..];

}

fn first_word_index(s: &String) -> usize {  // We take in a reference to a string
    let bytes = s.as_bytes(); // Convert string to bytes

    for (i, &item) in bytes.iter().enumerate() {  // For each (index and value) in the bytes
        if item == b' ' {  // item is a byte, so we must compare to a byte space character
            return i;
        }
    }

    return s.len();  // Return the end of the string if only 1 word
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..]
}
