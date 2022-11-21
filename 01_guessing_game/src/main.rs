use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Title.
    println!("=== GUESS THE NUMBER ===\n");

    // Generating the number.
    let generated_number: u32 = rand::thread_rng().gen_range(1..=100);

    // Counting the number of tries.
    let mut no_tries: u32 = 0;

    // Game loop.
    loop {
        // Asking the user for input.
        let mut user_input = input("Enter your guess");

        // Checking if the user is trying to quit.
        if user_input.to_lowercase() == "q" {
            println!("Bye!");
            break;
        }

        // Parsing the user input.
        let user_input: u32 = match user_input.trim().parse() {
            Ok(number) => {
                no_tries = no_tries + 1;
                number
            }
            Err(_) => continue,
        };

        // Giving feedback to the user.
        match user_input.cmp(&generated_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // Telling the player they won.
                println!("YOU WIN!");
                println!("Number of tries: {}", no_tries);
            }
        };
    }
}

pub fn input(message: &str) -> &str {
    println!("{}: ", message);
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read the input!");
    return user_input.truncate();
}
