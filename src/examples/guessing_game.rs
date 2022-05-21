use std::io;

pub fn guessing_game() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let trimmed = guess.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => println!("Your input integer: {}", i),
        Err(..) => println!("Not an integer: {}", trimmed),
    };
}