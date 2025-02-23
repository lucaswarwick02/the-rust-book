fn main() {
    /// Here we look at how functions handle scopes and memory. Take a look at
    /// the following two functions. `takes_ownership` takes in a String, and
    /// therfore `some_string` gets freed afterwards, so we cannot use it again.
    /// Because `makes_copy` takes in an i32 (stack variable)
    
    let s = String::from("hello");
    takes_ownership(s);  // Value moved to function, so no longer valid from here!

    let x = 5;
    makes_copy(x);

    println!("{x}");  // We can print!


    /// This bit handles returning values.
    
    let s1 = gives_ownership(); // Move value into v1

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // s2 is moved, so no longer scoped

    /// This bit shows how we can get a value back
    
    let s1 = String::from("hello");
    
    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is: {len}");

}  // s1 and s3 are out of scope now. s2 was moved so nothing happens

fn takes_ownership(some_string: String) {  // some_string comes into scope
    println!("{some_string}");
}  // some_string goes out of scope, dropped, and memory is freed.

fn makes_copy(some_integer: i32) {  // some_integer comes into scope
    println!("{some_integer}");
}  // some_integer goes out of scope, nothing special happens


fn gives_ownership() -> String {
    let some_string = String::from("yours");

    return some_string;  // Value gets passed, so we can use it when given
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    return a_string;  // Moves back out
}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    return (s, length);  // We can return the new value, *and* the old! 
}