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

    // Control flow
    // If-else expressions
    if x < 5 && x > 0 {
        println!("x is less than 5 and greater than 0");
    } else if x == 5 || x == 6 {
        println!("x is equal to 5 or 6");
    } else {
        println!("x is greater than 6");
    }

    // Assigning vars in if statements
    let condition = true;
    let number = if condition { 5 } else { 20 };

    println!("The value of number is: {}", number);

    // loops
    // infinite loop
    loop {
        println!("This will loop forever");
        break; // Remove this line to see the infinite loop
    }

    // returning values from loop 
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2; // Return value from the loop
        }
    };

    println!("The result of the loop is: {}", result);

    // loop labels
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer; // Breaks out of the outer loop
        }
        println!("This line will not be printed because we broke out of the outer loop");
    }

    // for loops
    // manually iterating over a range
    for i in 1..=5 {
        println!("The value is: {}", i);
    }

    // with .iter() method
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // while loops
    let mut n = 5;
    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }

    // match statements
    let number = 13;
    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        4 | 5 => println!("Four or Five!"),
        6..=10 => println!("Between Six and Ten!"),
        _ => println!("Something else!"), // Catch-all pattern
    }

    // destructuring with match
    let point = (3, 5);
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On the x-axis at {}", x),
        (0, y) => println!("On the y-axis at {}", y),
        (x, y) => println!("Point at ({}, {})", x, y),
    }
}