mod concatenate;
mod notes;

use concatenate::concatenate_strings;
use notes:: notes;

fn main() {
    notes();
    let string = String::from("Hello, World!");
    let string1: &str = &string[0..6];
    let string2: &str = &string[6..13];
    println!("---------------------------");
    println!("Concatenation String -> {}", concatenate_strings(string1, string2));
}