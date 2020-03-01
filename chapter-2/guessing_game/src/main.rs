use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number 💯");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Input your guess...");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line 💀"); //Expect outputs when crashing due to errors (this feels a bit weird but w/e)

        let guess: u32 = match guess.trim().parse() { // Parse returns Result type that is either enum Ok or Err 
            Ok(num) => num,
            Err(_) => continue,
        }; // Handle error

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small 🥜!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! 🔥");
                break;
            }
        };
    }
}
