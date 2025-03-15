#[allow(unused_variables, unused_mut)]
fn main() {
    let mut s = String::new();  // Same as vec

    let data = "initial contents";  // &str
    let s = data.to_string();  // String

    let s = "initial contents".to_string();  // or all at once

    let s = String::from("initial contents"); // Same as above!

    // Strings are UTF-8 encoded, so all of the following are valid:
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");  // Can directly append

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);  // Or appand a &str

    println!("s1 is {s1}");
    println!("s2 is {s2}"); // But we can still access it, as we didn't take ownership!

    // We can append chars using push
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {s}");

    // We can also concatenate with the `+` operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 is dereferenced here!

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;  // We can concatenate multiple, but still lose s1...

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");  // Or use the format! macro and keep everything!

    // We cannot using indexing with strings, due to how they are represented. 
    // They are wrappers around vec<u8>, with each charcter being two bytes.
    // This means s[0] actually gives the first byte, not the first character!
    // For example, the Hindi word "नमस्ते" 
    // is stored as:  [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    //                 224, 165, 135]
    // Which is 18 bytes, and also stored as ['न', 'म', 'स', '्', 'त', 'े'] in Unicode
    // Or as grapheme clusters: ["न", "म", "स्", "ते"]

    let hello = "Здравствуйте";
    let s = &hello[0..4];  // This prints the first 2 characters
    // let s = &s[0..1];   // This causes a panic!
    println!("{s}");

    // We can iterate through characters safely
    for c in "Зд".chars() {
        println!("{c}");  // Prints 2 times
    }

    // Or through bytes
    for b in "Зд".bytes() {
        println!("{b}");  // Prints 4 times
    }

}
