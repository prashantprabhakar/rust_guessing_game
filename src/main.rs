use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess a number!");

    let secret_num = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_num);



    loop {
        println!("Plase input your guess");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");
    
        println!("You guessed: {}", guess);
        
        // Parse the guess "String" to guess "u32";
        //let guess: u32 = guess.trim().parse().expect("Plase type a valid number");

        // Let's handle Error in parsing now:
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number");
                continue;
            },
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering:: Equal => {
                println!("You win!!");
                break;
            }
        }
        
    }

}