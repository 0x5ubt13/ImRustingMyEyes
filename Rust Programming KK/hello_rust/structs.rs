// structs organse data into a single unit, making the code more manageable
// structs can contain multiple data types

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Structs with non-copy fields
#[derive(Debug)] // This is used to print the struct in a human-readable format
struct Owner {
    name: String,
}

#[derive(Debug)] // This is used to print the struct in a human-readable format. If Owner doesn't have this, we will get an error too.
struct House {
    owner: Owner,
    rooms: u32,
}


fn main() {
    let height = 50;

    // create an instance of Rectangle
    let rect1 = Rectangle {
        width: 30,
        height, // using a field init shorthand
    };

    // print the area of the rectangle
    println!("The width of the rectangle is {} pixels.", rect1.width);
    println!("The height of the rectangle is {} pixels.", rect1.height);
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    // create another instance of Rectangle, using struct update syntax
    let rect2 = Rectangle {
        width: 10,
        ..rect1 // using the values from rect1 for the remaining fields
    };

    // print the area of the second rectangle
    println!("The width of the second rectangle is {} pixels.", rect2.width);
    println!("The height of the second rectangle is {} pixels.", rect2.height);
    println!("The area of the second rectangle is {} square pixels.", rect2.area());

    print_dimensions(&rect1);
    print_dimensions(&rect2);

    let mut rect3 = Rectangle {
        width: 5,
        height: 10,
    };

    modify_dimensions(&mut rect3);
    print_dimensions(&rect3);

    // Example of Non-Copy fields
    let owner1 = Owner {
        name: String::from("Alice"),
    };

    let house1 = House {
        owner: owner1, // Move ownership of owner1 into house1
        rooms: 3,
    };

    // println!("Owner: {}", owner1.name); // This will cause an error because owner1 has been moved into house1

    println!("House owner: {}", house1.owner.name);
    println!("House rooms: {}", house1.rooms);

    // Printing a struct in rust is done using the {:?} format specifier
    // If we don't implement the Debug trait for the struct, we will get an error:
    // error[E0277]: `House` doesn't implement `Debug`
    println!("House: {:?}", house1);
}

// Borrowing the Rectangle struct as a reference
fn print_dimensions(rect: &Rectangle) {
    println!("The width of the rectangle is {} pixels.", rect.width);
    println!("The height of the rectangle is {} pixels.", rect.height);
}

// Mutable borrowing of the Rectangle struct
fn modify_dimensions(rect: &mut Rectangle) {
    rect.width = 10;
    rect.height = 20;
}
