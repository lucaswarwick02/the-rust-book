#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),  // Can store an enum as a variable inside another enum
}

fn value_in_cents(coin: Coin) -> usize {
    return match coin {
        Coin::Penny =>  {
            println!("Lucky penny!");  // match control flow can execute block with a return
            return 1;
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            return 25;
        },
    };
}

fn plus_one(x: Option<usize>) -> Option<usize> {
    return match x {
        None => None,
        Some(i) => Some(i + 1),
    };
}

fn main() {
    let coin1 = Coin::Penny;
    let value1 = value_in_cents(coin1);

    let coin2 = Coin::Quarter(UsState::Alaska);
    let value2 = value_in_cents(coin2);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;

    // Here we have a bunch of example functions
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: usize) {}
    fn reroll() {}

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),  // Can use the default value
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),  // Allows us to reroll by default
    };

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => () // Or do nothing!
    }
}