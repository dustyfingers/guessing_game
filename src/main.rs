// the io library comes from rust's standard library
use std::io;

fn main() {
    println!("Guess The Number!");
    println!("Your grandma's new boyfriend's childhood dog's favorite computer game!");

    println!("");
    println!("");
    
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

    println!("");
    println!("");

    println!("You guessed: {guess}");
}
