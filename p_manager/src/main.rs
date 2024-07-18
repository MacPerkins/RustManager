use std::{io::{self, Write}, string};

fn greeting() { // Sends a greeting to the user
    println!("Welcome to Rust Password Manager!"); 
} 

fn build_menu() { // Builds the menu options 
    println!("-----MENU-----"); 
    println!("1. Add a Password"); 
    println!("2. Get Password"); 
    println!("3. List Accounts"); 
} 

fn main() { 
    greeting();
    loop { 
        build_menu();
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();

        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num, Err(_) => continue, };
            
        if choice == 1 { // Add Password
            println!("Adding password");
        } else if choice == 2 {  // Get Password
            println!("Getting password");
        } else if choice == 3 {  // List all accounts
            println!("Accounts: ");
        } else if choice == 4 {
            println!("Closing program. Goodbye!");
            break;
        }
    }
}