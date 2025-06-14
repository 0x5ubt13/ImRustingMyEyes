fn main() {
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
}