// Defining and using methods in structs

struct Book {
    title: String,
    pages: u32,
}

impl Book {
    // Method to calculate reading time assuming 1 page per minute
    fn reading_time(&self) -> u32 {
        self.pages
    }

    // Method to check if one book has more pages than another
    fn has_more_pages_than(&self, other: &Book) -> bool {
        self.pages > other.pages
    }

    // Associated function (not a method) to create a new book with default values
    fn default_book(pages: u32) -> Book {
        Book {
            title: String::from("Untitled"),
            pages,
        }
    }

    // Mutable method to change the title of the book
    fn change_title(&mut self, new_title: String) {
        self.title = new_title;
    }
}

fn main() {
    let book1 = Book {
        title: String::from("The Rust Programming Language"),
        pages: 550,
    };

    let book2 = Book {
        title: String::from("Programming in Rust"),
        pages: 300,
    };

    println!("Reading time for '{}': {} minutes", book1.title, book1.reading_time());
    println!("Reading time for '{}': {} minutes", book2.title, book2.reading_time());

    if book1.has_more_pages_than(&book2) {
        println!("'{}' has more pages than '{}'", book1.title, book2.title);
    } else {
        println!("'{}' does not have more pages than '{}'", book1.title, book2.title);
    }

    let mut default_book = Book::default_book(100);
    println!("Default book created: '{}' with {} pages", default_book.title, default_book.pages);

    default_book.change_title(String::from("New Title"));
    println!("After changing title: '{}'", default_book.title);
}