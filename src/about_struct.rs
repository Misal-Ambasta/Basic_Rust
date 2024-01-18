#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    publication_year: u32,
}

pub fn struct_notes() {

    // Unmutable struct
    let book = Book {
        title: String::from("Thats Coding"),
        author: String::from("Open source"),
        publication_year: 2001,
    };
    
    println!(
        "The book {} is written by {} in {}",
        book.title, book.author, book.publication_year
    );

    let mut book = Book {
        title: String::from("Thats Coding"),
        author: String::from("Open source"),
        publication_year: 2001,
    };

    book.publication_year = 2025;

    println!(
        "The book {} is written by {} in {}",
        book.title, book.author, book.publication_year
    );

    // Function to get Struct data

    let book_data = get_book_data(book);

    for data in book_data {
        println!("{data}");
    }


    // Create new book using Struct

    let my_book = create_book("The Path of Zen".to_string(), "Simon".to_string(), 2023);
    // Use #[derive(Debug)]to get below output
    println!("My book is {:?}", my_book);

    // tuple
    let book_tuple = BookNew("Some book".to_string(), "Simor".to_string(), 2024);
    
    let title = book_tuple.0;
    let author = book_tuple.1;
    let publication_year = book_tuple.2;
    
    println!("Tuple Book: {}, {}, {}", title, author, publication_year);

    // Unit Structs
    let empty_instance = Empty;
    empty_instance.greet();

    // Implementing Methods for Structs
    let my_rectangle = Rectangle {
        width: 10.0,
        height: 5.0,
    };
 
    let area = my_rectangle.area();
    println!("The area of the rectangle is: {}", area);

}

// Implementing Methods for Structs

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

}

 // Unit Structs 
struct Empty;

impl Empty {
    fn greet(&self) {
        println!("Hello, I am an empty struct!");
    }
}

// Tuple Structs: A tuple struct is similar to a regular struct, but its fields do not have names. Instead, they are accessed by their position.

struct BookNew(String, String, u32);

// ---------------
fn get_book_data(book: Book) -> [String; 3] {
    let title = book.title;
    let author = book.author;
    let publication_year = book.publication_year;

    let data = [ title, author, publication_year.to_string() ];

    data
}

fn create_book(title: String, author: String, publication_year: u32) -> Book {
    let book = Book {
        title,
        author,
        publication_year,
    };

    book
}