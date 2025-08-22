// Vectors are one of the most commonly used data structures in Rust.
// They are similar to arrays but can grow and shrink in size.
// Vectors are defined using the Vec<T> type, where T is the type of elements stored in the vector.

fn main() {
    // Creating a new, empty vector
    let mut numbers: Vec<i32> = Vec::new();

    // Adding elements to the vector
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    // Accessing elements in the vector
    println!("First element: {}", numbers[0]);
    println!("Second element: {}", numbers[1]);

    // Iterating over the elements in the vector
    for number in &numbers {
        println!("Number: {}", number);
    }

    // Removing the last element from the vector
    if let Some(last) = numbers.pop() {
        println!("Removed last element: {}", last);
    }

    // Checking the length of the vector
    println!("Vector length: {}", numbers.len());

    // Creating a vector with initial values
    let fruits = vec!["Apple", "Banana", "Cherry"];
    for fruit in &fruits {
        println!("Fruit: {}", fruit);
    }

    // To modify elements in the vector, we need to use mutable references
    let mut mutable_fruits = vec!["Apple", "Banana", "Cherry"];
    for fruit in &mut mutable_fruits {
        *fruit = match *fruit {
            "Apple" => "Green Apple",
            "Banana" => "Yellow Banana",
            "Cherry" => "Red Cherry",
            _ => fruit,
        };

        println!("Modified Fruit: {}", fruit);
}