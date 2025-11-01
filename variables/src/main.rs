fn main() {

    /*
        COMPOUND TYPES
        1. Tuples: have fixed length, once declared don't grow or shrink
    */
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    /*
        2. Arrays: every element of the array must have same type. Fixed length as well.
    */
    let a = [1,2,3,4,5]; // array with fixed size (stack - faster)
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [0; 5]; // 5 elements all set to 0 (zero) automatically
    let vec = vec![1,2,3,4,5]; // array dynamic size (stack pointing to heap - slower)

    
    // let t = true;

    // let f: bool = false; // with explicit type annotation

    // let result = 5.0/3.0;
    // println!("The result is this {result}"); // 1.666666. But also 1 if it's not float type

    // // addition
    // let sum = 5 + 10;

    // // subtraction
    // let difference = 95.5 - 4.3;

    // // multiplication
    // let product = 4 * 30;

    // // division - RUST truncates TOWARD ZERO to the nearest integer
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3; // Results in -1

    // // remainder
    // let remainder = 43 % 5;

    // let x = 2.0; // f64 by default

    // let y: f32 = 3.0; // f32 due to set

    // Dont' work
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // Correct shadowing changing the type
    // let mut spaces = "   ";
    // let spaces = spaces.len();


    // Shadowing creates a new variable with the same name
    // On another scope
    /*
        The value of x in the inner scope is: 8
        The value of x is: 6
    */
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x+2;
    //     println!("The value of x in the inner scope is: {x}");
    // } // shadow will end and 'x' is back to 6!
    // println!("The value of x is: {x}");

    // There are differences between contants and immutable/muttable
    // I cannot use 'mut' with constants, because contants are always
    // immutables. I can use 'const' instead of 'let' as well with its type
    // example 'cont THREE_HOUR: u32 = 60 * 3;
    
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
}
