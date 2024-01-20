
pub fn basic_results(){
    let div_result = divide(4,2);
    let res = div_result.expect("we crashed");

    // match div_result {
    //     Ok(v) => println!("{}", v),
    //     Err(v) => println!("{:?}", v),     
    // }

    // if div_result.is_ok() {
    //     println!("{}", div_result.unwrap());
    // }

    // println!("{}", div_result.unwrap()); 
    // println!("{}", div_result.unwrap_or(100)); // If error then it will show 100
    // println!("{}", res);
}

#[derive(Debug)]
enum MyError {
    Error1
}

//Err, an enum that contains an error code.
// Ok(value), A wrapper that contains a value.

pub fn divide(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if dividend % divisor != 0 {
        Err(MyError::Error1)
    } else {
        Ok( dividend / divisor)
    }
}