fn main() {
    let s1 = String::from("hello");  // Get an object as normal

    let len = calculate_length(&s1);  // Pass in reference to avoid ownership change

    println!("The lenth of '{s1}' is: {len}");

    let mut s = String::from("hello");

    change(&mut s);  // Pass in a mutable reference!

    /// Cannot have two mutable references to a value!
    /// let r1 = &mut s;
    /// let r2 = &mut s;
    /// This will through a compiler error
    
    let mut s = String::from("hello");
    {
        let r1 = &mut s; // This works
    }
    let r2 = &mut s; // But this does work! Different scopes

    /// fn main () {
    ///     let reference_to_nothing = dangle();
    /// }
    /// 
    /// fn dangle() -> &String {
    ///     let s = String::from("hello");
    /// 
    ///     return &s;
    /// }
    /// 
    /// The above code will through a compile time error, this is because after
    /// `dangle()`, s is out of scope and therfore freed, but `reference_to_nothing`
    /// has a reference to it? Impossible!
    println!("This line is needed to avoid an error lmao");
}

fn calculate_length(s: &String) -> usize {  // Get a reference, without taking ownership
    return s.len();
}

fn change(some_string: &mut String) {  // Takes in a mutable reference to a string!
    some_string.push_str(", world");  // Because its mutable, we can add to it
}