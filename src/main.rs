use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guessing game");

    let secret_number = rand::thread_rng().gen_range(1..=3);

    loop {
        println!("please input your guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
