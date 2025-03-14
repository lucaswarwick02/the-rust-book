fn main() {
    // Define an empty vector
    let v: Vec<i32> = Vec::new();  // Store multiple values, adjacent in memory, of the same type

    // Define a a Vector with values
    let v = vec![1, 2, 3];

    // Can update as we go
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    // We can directly get the element (unsafe)
    let third = &v[2];
    println!("The third element is {third}");  // This works, but may crash!

    // Or use `get` for an option (safe)
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element")
    };

    // Below is an example of attempting to add, whilst holding a reference 
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // Adding an element may require a memory location change
    // `first` may now point to unallocated memory
    // v.push(6);  // Error! 

    println!("The first element is {first}");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;  // If mutable, we cna update each value
    }

    // Using enums to store different types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
}
