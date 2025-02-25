use std::io;
use std::process;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the secret number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.  Or type \"quit\" to exit.");

        let mut guess = String::new(); // mutable


        /*
        String::new() - creates a new instance of a string type.
        :: - calls an associated function.
        new() - an associated function that creates a new string.
        */

        io::stdin()

            .read_line(&mut guess)
            // & means it's a references, which are immutable by default.
            /*
            & means it's a references, which are immutable by default.
            Returns a Result value, i.e. an enumeration/enum.
            An enum can be one of several variants: Ok or Err
            
            */

            
            .expect("Failed to read line."); 
            // If enum is Err, it will display the message passed to .expect()
        
        if guess.trim() == "quit" {
          println!("Goodbye!");
          process::exit(1);
        }
        // create shadow variable for guess that converts into a number.
        let guess: u32 = match guess
            .trim()  // eliminates white space at beginning and end
            .parse()  // convert string to u32 number
                {
                    Ok(num) => num,
                    Err(_) => continue,
                };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number ) {
            // Order has enums Less, Greater, and Equal
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
