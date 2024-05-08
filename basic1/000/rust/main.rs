/*
 * I will try the same process as Python.
 * First run is to send it to the CLI using print.
 * Then I'll run it with variables! It all works!
 */

// println!("Hello, World!");
// The above code has compile time errors because it lacks a main function
fn main() {
    println!("Hello, World!");

    let x: &str = "Hello,";
    let y: &str = "World!";

    // Should the above declaration be `let x: String`

    println!("{} {}", x, y);
}
