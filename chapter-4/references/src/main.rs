// Giving and returning ownership is a pain in the ass 
// Use references (&) instead
fn main() {
    let s1 = String::from("beep boop");

    let len = calculate_length(&s1);

    println!("The length of {} is {}", s1, len);

}

fn calculate_length(s: &String) -> usize { // GOTCHA: return type is a number/size not string! 
    s.len()
}
// NOTES: 
// References are immutable because the values are immutable. 
// A reference can be mutable if the value itself is explicitly marked as mut! 
// The opposite of a reference is a dereference (*)