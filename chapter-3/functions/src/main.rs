fn main() {
    another_function();
    params(5, 6);
    expression();

    find_x();
}

fn another_function() {
    println!("Beep boop");
}

// Types must be given for each argument!
fn params(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Statements vs Expressions
// S don't return a but value E do 
fn statement() {
    let y = 6;
    // ^ this doesn't return anything so you can't do this: 
    // let x = (let y = 6) 
    // Wild, but Ruby would let you pull this "x = y = 6" this and I'm not saying anything 👀
}
// ^ the block however, is an expression 

fn expression() {
    // These evaluate to something 
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // ⬅️ Note the lack of a ; at the end of the line 
                // Expressions DO NOT includen a last line ; 
                // If there is one the expression becomes a statement and doesn't return anything
    };
    println!("the value of y is: {}", y)
}

// Functions with return values
fn five() -> i32 {
    // You can return early from a function with the return keyword -- otherwise the last line return in implicit
    5
}

fn find_x() {
    let x = five();

    println!("We found x! x was {}", x);
}

