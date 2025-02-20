fn main() {
    // Rust has 4 scaler types, meaning they represent a single value
    // Integers, Floats, Booleans and Characters

    let x = 2.0;  // f64 (default)
    let y: f32 = 3.0; // f32

    // Additiom
    let sum = 5 + 10;

    // Subtraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // Remainder
    let remainer = 43 % 5;

    // Booleans
    let t = true;
    let f: bool = false;  // With explicit type annotation

    // Characters
    let c = 'Z';
    let z: char = 'Z';  // With explicit type annotation
    let heart_eyed_cat = '😻';

    // Compound types can group multiple types into one
    // Rust has 2 primitive compound types: Tuples and Arrays

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);  // Define type for each element
    let (x, y, z) = tup;  // Can unpack a tuple into its elements
    println!("The value of y is: {}", y);

    let one = tup.2;  // Can directly select elements by their index

    // Arrays have a fixed length. Like C#, unlike Python
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"];
    println!("There are {} months in the year", months.len());

    // Can set the type like so
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Can Set the default elements too
    let a = [3; 5]; // a = [3, 3, 3, 3, 3]

    let a = [1, 2, 3, 4, 5];
    let first = a[0];  // Can grab by index, but this time use square brackets
    let second = a[1];
}
