fn main() {
    let s = String::from("Hello"); // s is in scope

    takes_ownership(s); // value of s is moved here

    let x = 5; // x is in scope

    makes_copy(x); // copy made but value in x variable can still be used 
} // x goes out of scope

fn takes_ownership(str: String) {
    println!("{}", str)
}

fn makes_copy(int: i32) {
    println!("{}", int)
}

