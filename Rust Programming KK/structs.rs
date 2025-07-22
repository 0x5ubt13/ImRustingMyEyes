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
}