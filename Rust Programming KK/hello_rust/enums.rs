// An Enum is like a list of possible options that a value, or a variable, can be or take. These are called "variants".
// For example, directions in an array can be North, South, East, West, or any combination of these.
#[derive(Debug)] // This allows us to print the enum variants using {:?}
enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}   

// Built-in enums in Rust include Option and Result.
// Option is used for values that can be either something or nothing (like a nullable type).
enum Option<T> {
    Some(T),
    None,
}

// Result is used for values that can be either Ok (successful) or Err (error).
enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // Struct-like variant with fields
    Write(String), // Tuple-like variant with a single String field
    ChangeColor(i32, i32, i32), // Tuple-like variant with three i32 fields
}

// More complex enums:
enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}

fn sound(animal: Animal) -> &'static str {
    match animal {
        Animal::Dog(_) => "Woof!",
        Animal::Cat(_) => "Meow!",
        Animal::Bird(_) => "Tweet!",
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }

    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60, // Red light lasts 60 seconds
            TrafficLight::Yellow => 5, // Yellow light lasts 5 seconds
            TrafficLight::Green => 30, // Green light lasts 30 seconds
        }
    }
}

// Showcasing the option enum
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// Showcasing the result enum
use std::fs::File;
use std::io::ErrorKind;

// Example of using an enum
fn main() {
    let direction = Direction::NorthEast;
    match direction {
        Direction::North => println!("Heading North!"),
        Direction::South => println!("Heading South!"),
        Direction::East => println!("Heading East!"),
        Direction::West => println!("Heading West!"),
        Direction::NorthEast => println!("Heading North-East!"),
        Direction::NorthWest => println!("Heading North-West!"),
        Direction::SouthEast => println!("Heading South-East!"),
        Direction::SouthWest => println!("Heading South-West!"),
    }

    let dir2 = Direction::South;
    println!("Direction 2 is: {:?}", dir2);

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hiya!"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    // To use these enums, pattern matching is often used:
    match msg1 {
        Message::Quit => println!("Quit message received!"),
        Message::Move { x, y } => println!("Moving to coordinates: ({}, {})", x, y),
        Message::Write(text) => println!("Writing message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Changing color to RGB({}, {}, {})", r, g, b),
    }

    let animal1 = Animal::Dog(String::from("Buddy"));
    let animal2 = Animal::Cat(String::from("Whiskers"));
    let animal3 = Animal::Bird(String::from("Tweety"));

    println!("Dog sound: {}", sound(animal1));
    println!("Cat sound: {}", sound(animal2));
    println!("Bird sound: {}", sound(animal3));

    let light = TrafficLight::Red;
    println!("Current light: {:?}", light);
    println!("Next light: {:?}", light.next());
    println!("Duration of current light: {} seconds", light.duration());

    let result = divide(10.0, 2.0);
    match result {
        Option::Some(value) => println!("Result of division: {}", value),
        Option::None => println!("Cannot divide by zero!"),
    }

    let file_result = File::open("non_existent_file.txt");
    match file_result {
        Result::Ok(file) => println!("File opened successfully: {:?}", file),
        Result::Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("File not found!"),
            other_error => println!("An error occurred: {:?}", other_error),
        },
    }
}



