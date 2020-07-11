use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess What?");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Entered: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("go up"),
            Ordering::Greater => println!("go down"),
            Ordering::Equal => {
                println!("correct");
                break;
            }
        }
    }
}
