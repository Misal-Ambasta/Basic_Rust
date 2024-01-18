mod concatenate;
mod notes;
mod about_struct;
mod about_enum;

use concatenate::concatenate_strings;
use notes:: notes;
use about_struct:: struct_notes;
use about_enum:: enums_notes;

fn main() {
    notes();
    let string = String::from("Hello, World!");
    let string1: &str = &string[0..6];
    let string2: &str = &string[6..13];
    println!("---------------------------");
    println!("Concatenation String -> {}", concatenate_strings(string1, string2));
    println!();
    println!("------------STRUCT---------------");
    struct_notes();
    println!();
    println!("------------ENUMs---------------");
    enums_notes();
}