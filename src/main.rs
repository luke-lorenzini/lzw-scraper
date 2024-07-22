use demo::Thing;

use std::fs;

fn main() {
    // let message = fs::read_to_string("./input_text").expect("Unable to read file");
    let message = fs::read("./input_text").expect("Unable to read file");

    let res = Thing::stuff(message);

    println!("{:?}", res);
}
