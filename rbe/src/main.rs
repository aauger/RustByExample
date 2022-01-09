use std::{vec, string, ops::Add};

fn modify_string(input: String) -> String {
    input.add("world!")
}

fn main() {
    let vector = vec!["hello", "world"];
    let mut result = vector.join(" ");
    result = modify_string(result);
    println!("{}", result);
}
