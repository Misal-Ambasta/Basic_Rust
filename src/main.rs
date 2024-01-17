fn main() {
    println!("Hello, world!");
    let x: i32 = 33;
    let _pi: f64 = 3.14;
    let _is_true: bool = true;
    let _letter_a: char = 'a';

    if x > 0 {
        println!("x is non-negative")
    } else {
        println!("x is negative")
    }

    let y: i32 = 1;
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    let mut i = 1;

    while i <= 5 {
        println!("{}", i);
        i += 1;
    }
    println!("add {}", add(x, y));

    // String
    let _message: &str = "Hello, world!"; // unmutable
    let mut _name = String::from("Alice"); // mutable

    //Arrays
    let _numbers: [i32; 3] = [1, 2, 3];
    let _days_if_week: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thurusday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    let _first_element = _days_if_week[0];
    let _last_element = _days_if_week[_days_if_week.len() - 1];

    // Slices: which represent a variable-size sequence of elements of type T
    let slice = &_days_if_week[1..3];
    let _first_slice_element = slice[0];

    // Tuples : a fixed-size sequence of elements of different types
    let person = ("Alice", 30);

    let _name = person.0;
    let _age = person.1;

    let person = (("Alice", "Smith"), 30);
    println!(
        "The person's name is {} {} and their age is {}.",
        person.0 .0, person.0 .1, person.1
    );

    // Unit tupe:  which represents an empty tuple and is used when no value is needed
    let _unit_type = ();

    // What is the difference between a string slice and a string in Rust?
    // Answer: A string slice is a reference to a portion of a string, while a string is a collection of characters.

    // Functions
    //fn function_name(parameter1: type1, parameter2: type2) -> return_type {
    // Function body
    //   return value;
    // }

    fn add_numbers(x: i32, y: i32) -> i32 {
        let result = x + y;
        return result;
    }

    let sum = add_numbers(3, 5);
    print!("sum: {}", sum);

    // Default Parameters: Rust doesn't support default parameters in functions.
    // However, you can use the Option type to simulate default parameter behavior.

    fn _greet(name: Option<&str>) {
        match name {
            Some(n) => println!("Hello, {}!", n),
            None => println!("Hello, Rust!"),
        }
    }
    // print!("Greet: {}", greet(name));

    // Conditional Statements

    let x = 5;

    if x > 10 {
        println!("x is greater than 10");
    } else if x < 10 {
        println!("x is less than 10");
    } else {
        println!("x is equal to 10");
    }

    // While Loop

    let mut counter = 0;

    while counter < 5 {
        println!("While loop Counter value is {}", counter);
        counter += 1;
    }
    // For Loop

    let numbers = vec![1, 2, 3, 4, 5];

    for number in numbers {
        println!("For loop: Number is {}", number);
    }

    // Loop

    let mut counter = 0;

    loop {
        println!("Counter value is {}", counter);
        counter += 1;

        if counter == 5 {
            break;
        }
    }

    // Match : Switch statement

    let num = 5;

    match num {
        1 => println!("The number is one!"),
        2 => println!("The number is two!"),
        3 => println!("The number is three!"),
        _ => println!("The number is something else!"),
    }

    // Expression based
    let result = match num {
        1 => "The number is one!",
        2 => "The number is two!",
        3 => "The number is three!",
        _ => "The number is something else!",
    };

    println!("{}", result);

    // Strings/Arrays in Rust are dynamically allocated in memory determined at runtime.
    /*
    In Rust, memory is divided into two main categories: the stack and the heap.
    The stack is a region of memory that is used for static memory allocation,
    which means that the size of the memory is determined at compile-time.
    The heap, on the other hand, is a region of memory that is used for dynamic memory allocation,
    which means that the size of the memory can change at runtime.

    When the program exits the main function, Rust automatically deallocates the memory that was allocated for x, y, and z.
    This ensures that there are no memory leaks or other memory-related bugs.

    Ownership prevent common bugs such as null pointer errors, dangling pointers, and memory leaks
     */

    let x = 5; // stored on the stack
    let y = String::from("hello"); // stored on the heap
    let z = y; // ownership of y is moved to z
    println!("x = {}, z = {}", x, z);

    /*
    Borrowing allows you to temporarily access a value without taking ownership of it.
    When you borrow a value, you create a reference to it,
    which allows you to read or modify the value without becoming the owner.

    There are two types of references in Rust: mutable references and immutable references.
     */

    // Immutable References: you can have multiple immutable references to the same value at the same time. This is known as "aliasing".

    fn print_string(s: &String) {
        println!("{}", s);
    }

    let my_string = String::from("hello, world!");
    print_string(&my_string);

    // Mutable References


    let mut my_string = String::from("hello");

    change_string(&mut my_string);

    println!("{}", my_string); // prints "hello world"


    fn change_string(s: &mut String) {
        s.push_str(" world");
    }

    // Any number of immutable references to a variable at a time.
    let _first_immutable_ref = &my_string;
    let _second_immutable_ref = &my_string;

    println!(
        "First immutable reference value: {}, second immutable value: {}",
         _second_immutable_ref, _first_immutable_ref,
    );

    // 

    let _first_mutable_ref = &mut my_string;
    println!("First mutable reference {}", _first_mutable_ref);

    let _second_mutable_ref = &mut my_string;
    println!("Second mutable ref value {}", _second_mutable_ref);
    // If first borrow later used then it will give ERROR
    // println!("First mutable reference {}", _first_mutable_ref);  

    /*
    Dangling References: reference that points to a memory location that has been deallocated,
    causing unexpected behavior or a runtime error. This can happen when a reference is still in scope
    after the object it refers to has been dropped.
     */

    let new_string = String::from("new string");
    let _new_string_ref = return_reference(&new_string);

    let _newer_string = new_string;

    // Below line we give ERROR: new_string value Deallocated and allocated to newer_string.
    // println!("new string ref", new_string_ref);


    /*
        The Clone Trait: creates a deep copy of a value.
        When you call clone on a value, it returns a new value that is an independent instance
        of the original value but has the same data.
     */ 

    let original_str = String::from("Hello, world");
    let cloned_str = original_str.clone(); // Creates a deep copy of original_str.

    println!("Original string {}", original_str);
    println!("cloned string, {}", cloned_str);

    // Using Clone with Borrowing and References
    let modified_string = modify_string(&original_str);

    println!("modified_string: {}", modified_string); // "Hello, world! modified"


}

fn modify_string(s: &String) -> String {
    let mut cloned_string = s.clone();
    cloned_string.push_str(" modified");
    cloned_string
}

fn return_reference(some_string: &String) -> &String {
    return some_string;
}