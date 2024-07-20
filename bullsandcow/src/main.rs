use std::{cmp::Ordering, io::{self}};

use rand::Rng;
fn main() {
    println!("Hey there!! Welcome to Bulls and Cows . A Number guessing game");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut attempts = 0;

    loop {
        println!("Enter an input between 1 to 100");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter Valid Number");
                continue;
            }
        };

        attempts += 1;

        if input< 1 || input> 100 {
          println!("Enter a number between range 1 to 100");
        }

        match input.cmp(&secret_number){
            Ordering::Less=>{
                println!("Smaller than the number");
                if attempts >10 {
                    print!("You have tried {} times",attempts);
                }
            }
            Ordering::Equal=>{
                println!("You got this in {} !! Congratulations 🥳🥳🎊",attempts);
                if attempts >10 {
                    print!("You have tried {} times",attempts);
                }
                break;
            }
            Ordering::Greater=>{
                println!("Bigger than the number");
                if attempts >10 {
                    print!("You have tried {} times",attempts);
                }
            }
        }

        if attempts > 10{
            println!("You tried {} times. You lost the game !! ",attempts);
        }
    }
}
