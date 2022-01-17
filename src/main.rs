use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

  println!("Guess the number!");

  let secret_number: i32 = rand::thread_rng().gen_range(1, 101);
  
  let mut guess: String = String::new();

  io::stdin()
      .read_line(&mut guess)
      .expect("Error!");

  let guess: i32 = guess.trim()
                            .parse()
                            .expect("Only Integers.");
  
  println!("You guessed: {}", guess);
  
  println!("Secret Number: {}", secret_number);
  
  match guess.cmp(&secret_number){
    Ordering::Less => println!("Guess was too low."),
    Ordering::Greater => println!("Guess was too high."),
    Ordering::Equal => println!("Guess was correct."),
  }
}
