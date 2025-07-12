// References in rust are always valid, where pointers can be null or dangling.
// They act like pointers, but come with safety guarantees.
// Unlike pointers, references are always valid and cannot be null or dangling.

fn main() {
    let value = 42;

    // Immutable reference allows us to borrow a value without taking ownership and without modifying it.
    let reference = &value; // Immutable reference to value
    println!("The value is: {}", reference); // Can read the value through the reference

    let x = 10;
    let y = &x; // Immutable reference to x
    println!("The value of x is: {}", y); // Can read, but not modify x through y
    // Attempting to modify x through y would result in a compile-time error

    // Passing ownership vs borrowing
    let s1 = String::from("hello");

    let len = calculate_length(s1); // Passing ownership
    //println!("{}", s1); // Error: s1 is no longer valid if we had taken ownership
    println!("The length of the string is {}", len);

    let s2 = String::from("world");
    let len2 = calculate_length_borrowed(&s2); // Borrowing the string
    println!("The length of {} is {}", s2, len2); // s2 is still valid here because we borrowed it

    // We can, however, create mutable references to allow modification of the borrowed value.
    // There can only be one mutable reference to a value in a particular scope to prevent data races though.
    let mut s3 = String::from("hello");
    let r3 = &mut s3; // Mutable reference to s3
    r3.push_str(", world!"); // Modify the string through the mutable reference
    println!("Modified string: {}", r3); // Can read the modified value through the mutable reference

    
}

fn calculate_length(s: String) -> usize {
    s.len() // Returns the length of the string, taking ownership
}

fn calculate_length_borrowed(s: &String) -> usize {
    s.len() // Returns the length of the string without taking ownership
}