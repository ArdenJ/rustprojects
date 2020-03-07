fn main() {
    // Return values can also transfer ownership
    let s1 = gives_ownership(); // the value returned from gives_ownership is transferred into s1

    fn gives_ownership() -> String {
        let str = String::from("oioi"); // oioi is assign to the variable
        str // str is returned
    }

    let s2 = String::from("Boop") // s2 comes into scope

    let s3 = takes_and_gives_back(s2) // s2 is passed into takes_and_gives_back(). the return value of the fn is transferred into s3

    fn takes_and_gives_back(str: String) -> String { // str is accepted as an argument 
        str // str is an expression which is return from function
    }
}
