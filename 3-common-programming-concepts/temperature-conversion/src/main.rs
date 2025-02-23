fn main() {
    let celsius = 37.0;
    println!("{celsius} in farenheit is: {}", to_farenheit(celsius));

    let farenheit = 98.6;
    println!("{farenheit} in celcius is: {}", to_celsius(farenheit));
}

fn to_farenheit(c: f32) -> f32 {
    return c * (9.0 / 5.0) + 32.0;
}

fn to_celsius(f: f32) -> f32 {
    return (5.0 / 9.0) * (f - 32.0)
}