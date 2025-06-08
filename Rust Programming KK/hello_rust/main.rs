fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    let mut y = 5;

    let y = 6;
    println!("The value of y is: {}", y);

    let y = y + 1;
    println!("The value of y is: {}", y);

    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    println!("Please input something: ");
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!("decimal: {}", decimal);
    println!("hex: {}", hex);
    println!("octal: {}", octal);
    println!("binary: {}", binary);
    println!("byte: {}", byte);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x: {}", x);
    println!("y: {}", y);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;

    // Mutability
    let mut x = 10;
    println!("Initial value: {}", x);

    x = 20;
    println!("Final value: {}", x);

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
}