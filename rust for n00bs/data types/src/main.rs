// Data types go hand-in-hand with variables, so it's hard to cover them as separate topics.  Some concepts here may not make 100% sense until you complete the Variables module, so feel free to revisit this module again if you need to.
// Rust has two data type subsets called scalar and compound.  A scalar holds a single value, where a compound can hold multiple values (i.e. they're collections).
// Scalar types include integers, chars, and booleans; and compound types include arrays and tuples.
// Data types are also referred to as being "primitive".  These are built into Rust's standard library and are stored on the stack.  Rust allows you to created your own custom data structures, which are stored on the heap.

fn main() {
    // Scalar data type subset -> ints, chars and bools
    // Ints

    // An integer is whole number, -2, -1, 0, 1, 2, etc.
    // If we want an integer to be negative, it should be declared as a "signed" integer.  Otherwise, it can be "unsigned".  Unsigned integers can be larger than signed integers because we're not wasting a "sign" bit.
    // We can allocate different sizes for an integer in multiples of 8 bits:  8, 16, 32, 64 and 128 bits.  An unsigned 8-bit integer (u8) can be 0 to 255; whereas a signed 8-bit integer (i8) can be -128 to 127.

    // signed int
    let int : i8 = -20;l

    // unsigned int
    let uint : u8 = 20;

    // A floating point is a number that can have decimal places, 8.2, 3.14, etc.
    // There are only two types of floating point numbers:  f32 and f64, which are 32 bits and 64 bits in size respectively.  
    // They are also both signed - there are no unsigned floating point types in Rust.
    let float32 : f32 = 8.2;
    let float64 : f64 = 3.1416;

    // A bool is a true or false value.  
    // It's a fundamental type as most "decisions" in a program are made based on a boolean expression (something that evaluates to true or false).
    let boolean : bool = true;
    
    // A char is a single letter or number, represented by a number.  Those "numbers" are standardised in the ASCII and Unicode tables.  
    // Different languages allow different byte sizes for characters, from 1 to 4 bytes.  
    // Rust uses 4 bytes, which allows it to use any character in UTF-32.
    // A character is defined with single quotes.
    let character : char = 'F';
    
    // Compound data type subset -> arrays and tuples
    // The array and tuples are both types of collections.  
    // An array can hold multiple values of a single data type; whereas a tuple can hold multiple values but of various data types.
    // Both types are very fast to use at runtime, but they are fixed size. 

    // Arrays
    // To create an array, we can do something like:
    let array : [i32,5] = [1,2,3,4,5];

    // Manually initialising each value is not viable for large arrays.  In this case, we can set every element to the same initial value using the following syntax:
    let array2 : [i32, 1000] = [0; 1000]; // This will create an array of 1000 elements and set each value to 0.
    println!("{}", array2[3])

    // Tuples
    // A tuple is declared using regular parenthesise for both the data types and initial values
    // Instead of accessing a tuple index via square braces, we use the period, .
    let tuple : (&str, &str, i32) = ("Charles", "Dickens", 1812);
    println!("{} {} was born in {}.", tuple.0, tuple.1, tuple.2);

    // You may also use a concept called "deconstruction" to assign friendly variable names to your elements.
    let (first_name : &str, last_name : &str, dob : i32) = tuple;
    println!("{} {} was born in {}.", first_name, last_name, dob);

    // Strings and slices of strings (&str)
    // Rust has two types of strings:  String and &str (called a string slice).  
    // The main practical difference is that the String is mutable and &str is immutable.  
    // For this reason, String is always stored on the heap, whereas &str may be stored on the stack.  Mutability is covered in the Variables module.
    // A string slice can be declared using double quotes: 
    let slice : &str = "Charles Dickens";

    // A string slice may be converted to a String using the .to_string() or String::from() methods.
    let str : String = slice.to_string();
    // or
    let str : String = "Charles Dickens".to_string();
    // or
    let str : String = String::from("Charles Dickens");
    // or
    let str : String = String::from(slice);

    // Note that there is no real conversion happening here - we're just accessing the pointer to where the string lives in memory.
    // Several methods exist to concatenate Strings.  One is to use the format! macro.
    let first_name : &str = "Rasta";
    let last_name : &str = "Mouse";
    let full_name : String = format!("{} {}", first_name, last_name) 

    // Another is to add each into an array and call the concat() method.
    let first_name : &str = "Rasta";
    let last_name : &str = "Mouse";
    let full_name : String = [first_name, " ", last_name].concat();

    // If a String is mutable, you can call the push_str() method.
    let mut name : String = String::from("Rasta");
    name.push_str(" Mouse");

    println!(name);

    // The + operator can also be used, but only to append a string slice to a String.
    let first_name : String = "Rasta";
    let full_name : String = first_name + " Mouse";

    println!(full_name)

}
