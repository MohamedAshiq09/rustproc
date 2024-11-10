use rand::Rng;
use std::io;


fn main() {
    // boolean operators 
    //print!("1<4 is {}", 1<4);
    //print! ("1==5 is {}", 1==5);
    // lazy boolean and && operator 
    //print!("1==1 && 1>3 is {}", 1==1 && 1>3);
    // lazy || or operator 
    //print!("1==1 || 1>3 is {}", 1==1 || 1>3)
    // condition statement 
    //let cond = 2 < 4;
     //print! ("{}", cond);
    // comparing integer with float 
    //let cond2 = (2 as f32) < 4.4;
    //print!("{}",cond2);
    //let cond = 2 <4 ;
    //let cond2 = false && cond;
    //print!("{}", cond2);
    // rust not operator 
    //let cond = 2 < 4;
    //let cond2 = !(true && cond);
    //print!("{}",cond2);
    // rust conditional operator
    // let food = "rice";

    // if food == "rice" {
    //     print!("it's nice");
    // } else if food == "bread" {
    //     print! ("it's awesome");    
    // } else {
    //     print!("not bad");
    // }
    // let x = 10;
    // if x==10 {
    //     print!("valid");
    // }else if x == 5{
    //     print!("not valid");
    // }else{
    //     print!("no user exists");
    // }
    // let greeting = String :: from ("hello world");
    // print!("{}",greeting);
    // let gfga = String::from("hello world"); 
    // print!("{}",gfga);
    // let x = String::from("ashiq");
    // print!("{}",x);
    // let mut is_even = true ;
    //  if is_even{
    //     print!("is_even");
    //  } else if !is_even {
    //     print!("the number is odd");
    //  } else {
    //     print!("no number")
    //  }
    // for i in 0..10
    // {
    //     print!("{}",i);
    // }
    // let para = String::from("a shiq");
    // let first_word = get_first_word(para);
    // print!("the first word is : {}", 
    // let mut add: i64    = 0;
    // print!("the nutural numbers are :");
    // for i in 1..10{
    //     add += i;
    //     print!("{}", i);
    // }
    //  print!("{}",add);

// } 

// fn get_first_word (para: String) -> String {
//     let mut ans = String::from("");
//     for char in para.chars() {
//         ans.push_str(char.to_string().as_str());
//         if char == ' ' {
//             break;
//         }
//     }
//     return ans;
loop {
    let secret_number = rand::thread_rng().gen_range(1..=10);
    let mut attempts = 0;
    
    println!("Welcome to the Guessing Game!");
    println!("I have selected a number between 1 and 10. Try to guess it!");

    loop {
        println!("\nGuess the number between 1 and 10:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Validate input
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            },
        };

        attempts += 1;

        // Check if the guess is correct, too high, or too low
        if guess == secret_number {
            println!("Congratulations! You guessed the number in {} attempts.", attempts);
            break;
        } else if guess < secret_number {
            println!("Too low! Try again.");
        } else {
            println!("Too high! Try again.");
        }
    }

    // Ask if the user wants to play again
    println!("\nDo you want to play again? (y/n):");
    let mut play_again = String::new();
    io::stdin().read_line(&mut play_again).expect("Failed to read line");

    if play_again.trim().to_lowercase() != "y" {
        println!("Thanks for playing! Goodbye!");
        break;
    }
}
}