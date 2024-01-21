mod examples;

use std::io::{self, Write};
use std::collections::{BTreeMap};

type MiniApp = fn();

fn main() {
    let app_functions: BTreeMap<&str, fn()> = BTreeMap::from([
        ( "hello_world", examples::hello_world::hello_world as MiniApp),
        ( "thumbnailer", examples::thumbnailer::thumbnailer as MiniApp)
    ]);

    // Display the applications
    println!("Please choose an application to run:");
    for (index, app) in app_functions.keys().enumerate() {
        println!("{}. {}", index + 1, app);
    }

    print!("Enter the number of the application you wish to run: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    println!();

    // Parse the input and handle the choice
    match choice.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= app_functions.len() => {
            let app_name = app_functions.keys().nth(num - 1).unwrap();
            if let Some(function) = app_functions.get(app_name) {
                function(); // Call the selected function
            }
        }
        _ => println!("Invalid choice. Please enter a number between 1 and {}", app_functions.len()),
    }
}

