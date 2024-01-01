use std::io;
use rand::Rng;
fn main(){ 
    println!("Guess the Number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret Number is {secret_number}");
    println!("Please Enter you Guess: ");
    //declare the variable and bind it to a new string
    let mut guess = String::new();
    //get input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to Read the guess");
    //print the guess
    println!("You're guess is {guess}");

}
