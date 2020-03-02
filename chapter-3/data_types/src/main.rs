fn main() {
    // Scalar Types! 4: integers, floating-point numbers, booleans, and characters

    // Integer Types
    // Signed and unsigned variants of either 8bit, 16, 32, 64, 128, or arch length
    // (Signed refers to whether a number can be position or negative - if you can assume the number is posi use unsigned)

    // Floats
    // Single f32 or double f64

    // Booleans

    // Characters
    // Rust characters are 4 bytes in size representing a unicode scalar value 
    let z = 'z';
    let cat = '🙀';

    // -------------------- 
    // Compound types
    // These group values into a single type. Rust has two primitive compound types: tuples and arrays

    // Tuple
    // FIxed length - once declared they cannot grow or shrink
    // (Type annotations are optional)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Values in tuples can be accessed by destructuring through pattern matching 💙
    // What's even dreamier is that the compiler suggests prefixing unused variables with an underscore!
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    // can also use dot notation to access tuples at their index (rather than creating unused variables)
    let five_hundred = tup.0;
    println!("five hundred is: {}", five_hundred);

    // Arrays
    // Arrays have a fixed length (this make me anxious - apparently vectors can be used when you want to growwww)
    // Every element in an array MUST be the same type
    // Type annotations specify both the type and length of the array as below
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let [a, b, c, d, e] = arr; // !! Array destructuring is a thing but you can also you access items at their index 
    println!("{}", a); 

    // You cn create an array that has the same value in each space 
    let same_value = [3; 5] // this evaluates to [3, 3, 3, 3, 3]

}
