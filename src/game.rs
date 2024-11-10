// use rand::Rng;
// use std::io;

// fn main() {
//     loop {
//         let secret_number = rand::thread_rng().gen_range(1..=10);
//         let mut attempts = 0;
        
//         println!("Welcome to the Guessing Game!");
//         println!("I have selected a number between 1 and 10. Try to guess it!");

//         loop {
//             println!("\nGuess the number between 1 and 10:");

//             let mut guess = String::new();
//             io::stdin().read_line(&mut guess).expect("Failed to read line");

//             // Validate input
//             let guess: i32 = match guess.trim().parse() {
//                 Ok(num) => num,
//                 Err(_) => {
//                     println!("Please enter a valid number.");
//                     continue;
//                 },
//             };

//             attempts += 1;

//             // Check if the guess is correct, too high, or too low
//             if guess == secret_number {
//                 println!("Congratulations! You guessed the number in {} attempts.", attempts);
//                 break;
//             } else if guess < secret_number {
//                 println!("Too low! Try again.");
//             } else {
//                 println!("Too high! Try again.");
//             }
//         }

//         // Ask if the user wants to play again
//         println!("\nDo you want to play again? (y/n):");
//         let mut play_again = String::new();
//         io::stdin().read_line(&mut play_again).expect("Failed to read line");

//         if play_again.trim().to_lowercase() != "y" {
//             println!("Thanks for playing! Goodbye!");
//             break;
//         }
//     }
// }
