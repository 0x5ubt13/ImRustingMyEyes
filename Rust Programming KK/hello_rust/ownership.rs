fn main() {
    let s = String::from("hello"); // Strings are heap-allocated and own their data
    println!("{}", s); // s is valid here
    takes_ownership(s); // Ownership is transferred to the function
    // println!("{}", s); // Error: s is no longer valid

    let x = 5; // Integers are stack-allocated and have the Copy trait
    makes_copy(x); // x is copied, not moved
    println!("{}", x); // x is still valid

    let s1 = gives_ownership(); // Function returns ownership of a String
    let s2 = String::from("hello");
    let s3 takes_and_gives_back(s2); // Ownership is transferred and returned
    //println!("{}", s2); // Error: s2 is no longer valid
    println!("{}", s3); // s3 is valid here

    // Demonstrating ownership transfer and modification
    let to_modify = String::from("hello");
    let result = modify_string(s); // to_modify is moved into the function
    println!("{}", result); // result now owns the modified string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // Ownership is returned through implicit return
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("{}", a_string);
    a_string // Ownership is returned
}

fn modify_string(mut s: String) -> String { // takes ownership of to_modify, modifies it, and returns it
    s.push_str(", world!"); // Modify the string in place
    s // Return the modified string via implicit return
}