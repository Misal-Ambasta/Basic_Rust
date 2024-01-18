pub fn enums_notes() {
    /*
        Enums: represent multiple related values within a single data type. 
    
     */

    let _current_weather = Weather::Sunny;

    // Enums with Associated Data
    let msg = Message::Write(String::from("Hello, Rust!"));

    process_message(msg);

    // The ‘if let’ Syntax
    //  a convenient and concise way to handle a single enum variant.
    // It can make your code easier to read and write when dealing with a specific case.

    let my_pet = Animal::Cat("Fluffy".to_string());

    if let Animal::Cat(name) = my_pet {
        println!("My cat's name is: {}", name);
    } else {
        println!("My pet is not a cat.");
    }

    //Enums and Methods in Rust
    let msg = Message::Write(String::from("Hello, Message!"));
    msg.call();
    
}

// Enums and Methods in Rust
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => println!("Change color to R: {}, G: {}, B: {}", r, g, b),
        }
    }
}

// The ‘if let’ Syntax
enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}

// Enums
enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
}

// Enums with Associated Data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor( i32, i32, i32 ),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data.");
        }
        Message::Move { x, y } => {
            println!("Move to coordinate x: {}, y: {}", x, y);
        }
        Message::Write(text) => {
            println!("Text Message: {}", text);
        }
        Message::ChangeColor(r, g, b ) => {
            println!("Change the color to red: {}, green: {}, blue: {}", r, g, b);
        }
    }
}