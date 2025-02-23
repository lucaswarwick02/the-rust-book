use std::io;


fn main() {
    loop {
        let mut n = String::new();

        // Read standard input and cast to int
        io::stdin().read_line(&mut n).expect("Failed to read line!");
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let fib_n = fibonacci(n);

        println!("The nth Fibonacci number is: {fib_n}");
    }
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}