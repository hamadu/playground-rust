extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("sec: {}", secret_number);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("fail");
        let guess: u32 = guess.trim().parse().expect("fail2");

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
