// Data types go hand-in-hand with variables, so it's hard to cover them as separate topics.  Some concepts here may not make 100% sense until you complete the Variables module, so feel free to revisit this module again if you need to.
// 
// Rust has two data type subsets called scalar and compound.  A scalar holds a single value, where a compound can hold multiple values (i.e. they're collections).
// 
// Scalar types include integers, chars, and booleans; and compound types include arrays and tuples.
// 
// Data types are also referred to as being "primitive".  These are built into Rust's standard library and are stored on the stack.  Rust allows you to created your own custom data structures, which are stored on the heap.

fn main() {
    // signed int
    let int : i8 = -20;

    // unsigned int
    let uint : u8 = 20;
}
