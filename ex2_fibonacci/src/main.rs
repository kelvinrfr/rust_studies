fn main() {
    println!("Started.");
    let n = 10;
    let result = fibonacci(n);
    println!("The result: {}", result);
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
