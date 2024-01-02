# Rust Guessing Game
This repo uses fundamental concepts in Rust to develop a Guessing Number Game.
Currently, it is a small game but it is intended that with time a gradual process of augmenting the game features is followed as one learns more and more about the language.
# Concepts Covered
- Main function `fn main(){}`
- Standard Crates `use std::io;`
- Print macro `prinln!("Guessing Game");`
- Variables and Mutability `let mut guess; //Variables are Immutable by default in Rust`
- Random Number Generation
```Rust
use rand::Rng;
fn main(){
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Random Number Generated is {random_number}");
}
```
- Getting User Input and Passing arguments to a function by reference
```Rust
use std::io;
fn main(){
    let mut guess = String::new(); //declaring a mutable guess Variable and binding it to new instance of String time
    //new is a Associate Function of Type String as it implements a function in a Type

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read number!");
}
```
-  Conditional Match and Loop
```Rust
use std::cmp::Ordering;
fn main(){
    //---snip---
    loop{
        match guess.cmp(&random_number){
            Ordering::Less => println!("Your guess is too Small!"),
            Ordering::Greater => println!("Your guess is too Big"),
            Ordering::Equal => {
                 println!("You Win!");
                 break;
                 }
        }
    }

}
```
