use std::io;

fn main() {
    println!("n?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    let output = fib(n);
    println!("fib(n) = {}", output);
}

// simple fibonachi function
fn fib(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fib(n - 2) + fib(n - 1)
    }
}
