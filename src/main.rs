// the io library comes from rust's standard library
use std::io;
use std::cmp::Ordering;

// pull random number gen methods from rand crate
use rand::Rng;

fn main() {
    println!("Guess The Number!");
    println!("Your grandma's new boyfriend's childhood dog's favorite computer game!");

    println!("");
    println!("");

    // generate secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess:");

        // set aside the string's space in memory
        // vars are immutable by default in rust, so we must use the 
        // mut keyword if we want the change the value contained in the variable
        let mut guess = String::new();
    
        // // so this is okay
        // guess = "".to_string();
    
        // // immutable
        // let apples = 5;
    
        // // but this is not
        // apples = 2;
    
        // this is where we actually read a line of input from the user
        io::stdin()
            // read_line() needs a variable to assign its output to
            // so we pass a mutable reference to guess using the & 'reference' sign and the mut keyword
            // so &mut guess instead of just &guess
            .read_line(&mut guess)
            // read_line() assigns the values of the passed in variable to be the users input - but it also 
            // returns a 'Result', which is an enum of either 'Ok' or 'Err'
            // if the result is an instance of an 'Err' , .expect() will will crash the program and 
            // display the message you passed as an argument  
            .expect("Failed to read line");
    
        // convert guess to number type for comparison
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            // 'continue' only breaks out of this iteration of the loop and starts the next iteration
            Err(_) => continue,
        };
    
        println!("");
        println!("");
    
        println!("You guessed: {guess}");
    
        // use a match statement to determine what to do next based on what variant of the 'Ordering' enum 
        // was returned from the .cmp() method
        match guess.cmp(&secret_number) {
            // each of these is an 'arm' of the match statement
            // an arm looks like this:
            // PatternToMatchAgainst => code_to_be_run_if_returned_value_matches_that_pattern()
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // 'break' breaks out of the entire execution of the loop
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
