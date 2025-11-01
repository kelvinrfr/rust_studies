fn main() {

    // We can also use FOR with RANGE to perform a countdown
    for number in (1..4).rev() { // 'rev' used to reverse the iterator (3,2,1)
        println!("{number}");
    }
    println!("LIFTOFF!!!");

    // let a = [10,20,30,40,50];
    // let mut index = 0;

    // for element in a {
    //     println!("the value is {}", element);
    // }

    // while index < 5 {
    //     println!("the value is {}", a[index]);
    //     index += 1;
    // }

    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}");
    //     number -= 1;
    // }
    // println!("LIFTOFF!!!");

    // Labeling counts - breaking outer loop
    // let mut count = 0;

    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;
        
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up; // breaks the outer loop
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {count}");

    // Loops
    // let mut counter = 0;
    
    // let result = loop { // creates a loop expecting an Expression to be resolved
    //     counter += 1;

    //     if counter == 10 {
    //         // return; - cannot be used - it ALWAYS exit function, not loop
    //         break counter * 2; // end the loop and return a value
    //     }
    // };
    // println!("The result is {result}");


    // let condition = true;
    // let number = if condition {6} else {3}; // inline condition

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }
}
