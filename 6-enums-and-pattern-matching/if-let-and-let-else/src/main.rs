#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Quarter(UsState),
}


fn main() {
    // If we want to run some code with only one match, and not do anything
    // for other conditions, we can do the following

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => () // This is annoying
    }

    // We can instead use let inside an if-else statement
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // We can use what we learnt previously to run default code
    let mut count = 0;
    let coin: Coin = Coin::Quarter(UsState::Alaska);
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //     _ => count += 1,
    // };

    // Or we can use the if-let-and-let-else
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}
