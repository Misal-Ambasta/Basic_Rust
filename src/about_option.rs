/* 
    Options
    Some(vaule), a tuple struct that wraps a value with type T.
    None, to indicate failure or lack of value.
    'If else' or 'match' can be used for None/Some
*/
pub fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some( dividend / divisor)
    }
}