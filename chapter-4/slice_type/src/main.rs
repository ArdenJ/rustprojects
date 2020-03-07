fn main() {
    // Problem: write a function that takes a string and returns the first word. if the string is only one word long: return the entire string

    // Initial thought would be to take in a reference to a  string and return the first word directly (but the Book says that this is wrong 😢)
    // fn first_word(s: &String) -> String {
    //     ...
    // }

    // THE BOOK's suggestion 
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes(); //Go through string to check for spaces by first converting it to an array of bytes 

        // create an iterator - destructured into the index and a reference to the item
        for (i, &item) in bytes.iter().enumerate() { // iter() returns each; enumerate() returns a tuple with the index of each
            if item == b' ' { // search for the first byte that matches the byr literal syntax b' ' 
                return i // exit the loop when the first space is found
            }
        }

        s.len() // if a space is found this code isn't reached! if a space isn't found the length of the string is returned 
    }

    let how_long = String::from("is the first word in this strong");
    let first = first_word(&how_long);
    println!("{}", first);
    // problem: the above returns a usize but this is only meaningful in reference and because it is a seperate value we can't garantee that is valid in the future

    // fn problem_example() {
    //     let mut s = String::from("eeeek");
    //     let word = first_word(&s); // word = 5
    //     s.clear(); // woahhh shit s now = "" BUT BUT BUT word still = 5
    // }


    // STRING SLICES 
    // A string slice is a reference to part of a String 

    fn slicey_does_it() {
        let s = String::from("Hello world");

        let hello = &s[0..5];
        let world = &s[6..11];
    }

    // using the slice method we can return a reference to the word in first_word()
    // directly by using a reference slice &s[0..i] or, if we don't find a space &s[0..] 
    // (a short-hand syntax for end of) ORRRRRRR even slicker &s[..] (shorter syntax for 
    // from the beginning to the end)
}
