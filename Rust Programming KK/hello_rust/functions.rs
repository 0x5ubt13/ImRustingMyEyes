fn main() {
    greet("John");
    let area = calculate_area(10.0, 20.0);
    println!("The area is: {}", area);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn calculate_area(length: f64, width: f64) -> f64 {
    // implicit return
    length * width
}

