fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelve"];
    let gifts = [
        "partridge in a pear tree.",  // Requires a conditional prefix
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas,", days[day]);
        println!("my true love gave to me");

        for gift in (0..(day+1)).rev() {
            let prefix = if gift == 0 {if day == 0 {"A "} else {"And a "}} else {""};

            println!("{}{}", prefix, gifts[gift]);
        }

        println!("");  // Space between each day
    }
}
