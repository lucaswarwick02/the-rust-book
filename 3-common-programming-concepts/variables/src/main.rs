fn main() {
    let mut x = 5;  // Must use mut in order for this variable to be changed in the future
    println!("The value of x is: {}", x);

    x = 6;  // Can change the contents of it, because its mut
    println!("The value of x is: {}", x);

    // Same as constants in other languages, but in rust they cannot use the mut keyword
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THe number of seconds in 3 hours is: {}", THREE_HOURS_IN_SECONDS);

    // Set the value of x to 5, then 6
    let x = 5;
    let x = x + 1;  // Using let allows us to "shadow" the first value, even though it wasnt mutatable!

    {
        let x = x * 2;
        println!("The value of x within the block is: {}", x);
    }

    println!("But the value of x outside the block afterwards is: {}!", x);

    // Shadowing even lets us change the type, with mut doesn't
    let spaces = "   ";
    let spaces = spaces.len();
}