fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    }
    else {
        println!("Condition was false");
    }

    let number = 3;

    if number != 0 {
        println!("Number was something other than zero");
    }

    // Handeling Multiple Conditions with else if
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    }
    else if number % 3 == 0 {
        println!("Number is divisible by 3");
    }
    else if number % 2 == 0 {
        // Not reached as it has already returned
        println!("Number is divisible by 2");
    }
    else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };  // Inline tertiary statement

    println!("The value of number is {number}");

    // This will infinitely loop until told to stop
    // loop {
    //     println!("Again!");
    // }

    let mut counter: u8 = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // Break exits current loop, return exits function
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;  // Exit loop and increase count
            }
            if count == 2 {
                break 'counting_up;  // Fully exit and stop looping
            }
            remaining -= 1;
        }

        count += 1;
    }

    // We exit out of everything when count == 2
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("Liftoff!");

    // Looping through array using while and index
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is {element}");
    }

    for number in (1..4).rev() {  // [1, 2, 3] but reversed!
        println!("{number}!");
    }
    println!("Liftoff!");
}
