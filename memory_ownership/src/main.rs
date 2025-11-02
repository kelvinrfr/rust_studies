fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");

    // As the immutable r1 and r2 are not being used anymore after println
    // there is no errors on creating a muttable reference from 's'
    // they are all borrowing the 's' reference, but sequence matters
    // as the reference might still be in use
}

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}.");

//     let mut hello = String::from("hello");
//     change(&mut hello);
//     println!("{hello}");
// }

// // the action of creating a reference is called 'borrowing'
// // because this function don't 'own' the received value
// fn calculate_length(s: &String) -> usize {
//     // s.push_str("error"); - references are immutables by default
//     s.len()
// }

// // it is possible to borrow a value and update that value
// // but need to be explicit in the function signature
// fn change(some_string: &mut String) {
//     some_string.push_str(", world.");
// }

// fn main() {
//     let s1 = gives_ownership();        // gives_ownership moves its return
//                                        // value into s1

//     let s2 = String::from("hello");    // s2 comes into scope

//     let s3 = takes_and_gives_back(s2); // s2 is moved into
//                                        // takes_and_gives_back, which also
//                                        // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {       // gives_ownership will move its
//                                        // return value into the function
//                                        // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string                        // some_string is returned and
//                                        // moves out to the calling
//                                        // function
// }

// // This function takes a String and returns a String.
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string comes into
//     // scope

//     a_string  // a_string is returned and moves out to the calling function
// }

// fn main() {
//     let mut z = String::from("hello");
//     // when a new value is assigned to an existing variable,
//     // Rust will call 'drop' and free the original value of previous
//     // memory allocated before
//     z = String::from("ahoy");


//     let s1 = String::from("hello");
//     let s2 = s1;
//     // this causes an error because s1 does not exists anymore
//     // it's content was MOVED to s2 instead.
//     // the pointer was copied but heap memory remains the same
//     println!("{s1}, world!"); 

//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     // Now this works because everything is being copied
//     // The pointer and the heap value
//     // this is more expensive and can lead to problems to memory
//     // Use with caution
//     println!("s1={s1}, s2={s2}");


//     // literals are immutable, because of that they are fast
//     // but it cannot put blobs in memory
//     let s = "hello"; // can be replaced but not updated

//     // String can store an amount of text that is unkown
//     // at compile time
//     let mut x = String::from("hello");

//     // This kind of string can be mutated
//     x.push_str(", world!"); // appends a literal to a String
//     println!("{x}");
// }

// fn main_two() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // Because i32 implements the Copy trait,
//                                     // x does NOT move into the function,
//                                     // so it's okay to use x afterward.

// } // Here, x goes out of scope, then s. However, because s's value was moved,
//   // nothing special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.