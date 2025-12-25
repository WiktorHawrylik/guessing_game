use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let secret_nmber = rand::thread_rng().gen_range(1..=100);

    println!("Guesss the number between 1 and 100!");

    loop {
        println!("What's your shot baby?");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess:i32 = guess.trim().parse().expect("Not a number!!");

        match guess.cmp(&secret_nmber){
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("Baby guessed!!! Baby wins!");
                break;
            }
        }
    }
}
