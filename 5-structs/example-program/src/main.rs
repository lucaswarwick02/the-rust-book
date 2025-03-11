#[derive(Debug)]  // Allows us to print the object (__repr__/__str__ in Python)
struct Rectangle {
    width: usize,
    height: usize
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectable in {} in square pixels.", area(width1, height1));

    let dimensions = (30, 50);
    println!("The area using tuples is {}.", area_tuple(dimensions));

    let rectangle = Rectangle {
        width: 30,
        height: 50
    };
    println!("The area using structs is {}.", area_struct(&rectangle));

    println!("The rectangle is {rectangle:?}.");  // `:?` prints using the Debug formatting

    dbg!(&rectangle);  // Logs to stderr, with the formatting AND line of file

    let scale = 5;
    dbg!(scale * 6);  // This will print `[src\main.rs:26:5] scale * 6 = 30` to stderr
}

fn area(width: usize, height: usize) -> usize {
    return width * height;
}

fn area_tuple(dimensions: (usize, usize)) -> usize {
    return dimensions.0 * dimensions.1;
}

fn area_struct (rectangle: &Rectangle) -> usize {
    return rectangle.width * rectangle.height;
}