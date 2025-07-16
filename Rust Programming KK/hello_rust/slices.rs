fn main() {
    // Slices in Rust are a view into a contiguous sequence of elements, similar to arrays but without ownership.
    // They are used to borrow a portion of an array or a vector without taking ownership.
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; // Borrowing a slice of the array
    println!("{:?}", slice); // Output: [2, 3, 4]


    // Slicing a string
    let s = String::from("Hello, world!");
    let hello = &s[0..5]; // Borrowing a slice of the string
    let world = &s[7..12]; // Another slice of the string
    println!("{} {}", hello, world); // Output: Hello world

    // Mutable slices
    let mut vec = vec![1, 2, 3, 4, 5];
    let slice_mut = &mut vec[1..4]; // Mutable slice of the vector
    slice_mut[0] = 20; // Modifying the first element of the mutable slice
    println!("{:?}", vec); // Output: [1, 20, 3,
}