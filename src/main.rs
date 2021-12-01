mod day1;
mod common;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let star = &args[2];
    let filename = &args[3];
    
    match day.as_str() {
        "1" => day1::day1(star, filename),
        _ => println!("Error: Unknown day!"),
    }
}
