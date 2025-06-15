fn main() {
    // The sring type in Rust requires heap allocation to support mutable, growable text, as its memory size is unknown at compile time.
    // String literal is a fixed-size string that is stored on the stack.
    // String object is a dynamic string that is stored on the heap.

    // String literal
    let s = "Hello";
    println!("{}", s);

    // String literal
    let mut s = String::from("Hello");
    println!("{}", s);

    s = "world";
    println!("{}", s);

    // string object
    let mut sobj = String::from("hello");
    println!("{}", sobj);

    sobj.push_str(" world");
    println!("{}", sobj);

    // Depending on the traits, data can be copied (shallow copy) or cloned (deep copy)
    // Using copy
    let x2 = 5;
    let y2 = x2; // x2 is copied to y2
    println!("x2: {}, y2: {}", x2, y2);

    // Using clone
    let x3 = String::from("hello");
    let y3 = x3.clone(); // explicitly clone the data
    println!("x3: {}, y3: {}", x3, y3); // both are valid
    

}