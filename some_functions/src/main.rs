fn main() {
    println!("Hello, world!");

    let x = plus_one(5);
    println!("The value of x is: {x}");

    // let y = {
    //     let x = 3;
    //     x + 1 // intentionally without semicollon
    //     // without semicollon means it is a Expression and can
    //     // Be returned and assigned to y.. WTF
    // };

    // println!("The value of y is: {y}");
    // Statements - functions does not return a value
    // Expression - results into a value
    // let y = 6; // this is a statement

    // let x = (let y = 6); - (statement) this is invalid as y=6 does not return value
    // 6+5 - (expression) - returns a value that binds

    // another_function(y,'h');
}

fn plus_one(x: i32) -> i32 {
    x + 1 // Expression to be returned
}

// fn five() -> i32 {
//     5 // Expression to be returned
// }

// order don't matter, but should be visible for caller
// fn another_function(x: i32, unit_label:char) {
//     println!("The value of x is: {x}{unit_label}");
// }
