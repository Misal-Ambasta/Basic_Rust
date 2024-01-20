mod concatenate;
mod notes;
mod about_struct;
mod about_enum;
mod about_option;
mod about_HashMap;
mod about_results;

use concatenate::concatenate_strings;
use notes:: notes;
use about_struct:: struct_notes;
use about_enum:: enums_notes;
use about_option:: divide;
use about_HashMap:: basic_HashMap;
use about_results:: basic_results;

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
    println!("------------OPTION---------------");
    let divide1 = divide(4, 2);
    let _divide2 = divide(2, 3);

    // Unwarpping a 'Some' variant will extract the value wrapped.
    println!("{:?} unwraps to {}", divide1, divide1.unwrap());

    // Unwrapping a 'None' variant will 'panic'.
    // println!("{:?} unwraps to {}", _divide2, _divide2.unwrap());

    println!("------------HASHMAP---------------");
    basic_HashMap();

    println!("------------Results---------------");
    basic_results();

}