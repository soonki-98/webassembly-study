extern crate rand;

use std::io; // std -> 표준 라이브러리, :: -> 연관함수
// use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // 1 <= x < 101
    let mut chance = 10;

    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); 

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        chance -= 1;

        let result = guess == secret_number;

        if result == true {
            println!("You win!"); 
            println!("Your score is {}", chance * 10);
            break;
        } else if guess < secret_number {
            println!("Too small. chance = {chance}");
        } else {
            println!("Too big!. chance = {chance}");
        }

        if chance == 0 {
            println!("You failed!!!");
            println!("answer is {secret_number}");
            println!("Your score is {}", chance * 10);
            break;
        }
        // match guess.cmp(&secret_number) {
        //     Ordering::Less    => println!("Too small!"),
        //     Ordering::Greater => println!("Too big!"),
        //     Ordering::Equal   => {
        //         println!("You win!");
        //         break;
        //     }
        // }
    }
 
}