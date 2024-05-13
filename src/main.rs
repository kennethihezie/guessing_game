use std::io;

/*
Macros in Rust are a way of defining reusable chunks of code. They are similar to functions, 
but instead of generating a value, they generate code. This means that they are evaluated 
at compile time, not at runtime, which can lead to more efficient code.
 */
fn main() {
   println!("Guess the number");

   println!("Please input your guess.");

   /* In Rust, variables are immutable by default, 
   meaning once we give the variable a value, 
   the value won’t change. To make a variable 
   mutable, we add mut before the variable name: */
   let mut guess = String::new();

   io::stdin()
   .read_line(&mut guess)
   .expect("Failed to read line");

   println!("You guessed: {guess}")
}
