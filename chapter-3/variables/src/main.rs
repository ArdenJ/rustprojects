fn main() {

    // Mutable variable declarations (You can't mutate a variable's type 🙅)
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants (ALL_CAPS - can't be mutated and require explicit type annotation)
    const CONTANT_X: u32 = 9000;
    println!("CONSTANT_X is: {}", CONTANT_X);

    // Shadowing variables
    let y = 5; // 5
    let y = y + 1; // 6 = 5 + 1
    let y = y * 2; // 12 = 6 * 2
    println!("The value of y is: {}", y);
  }