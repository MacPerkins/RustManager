use std::{io::{self, Write}, string};

struct PasswordManager {
    passwords: Vec<(String, String)>,
}

impl PasswordManager {
    // Constructor
    fn new() -> Self {
        PasswordManager {
            passwords: Vec::new(),
        }
    }

    // Function to add a new password
    fn add_password(&mut self, account: String, password: String) {
        self.passwords.push((account, password));
    }

    // Function to retrieve a password
    fn get_password(&self, account: &str) -> Option<&String> {
        for (acc, pwd) in &self.passwords {
            if acc == account {
                return Some(pwd);
            }
        }
        None
    }

    // Function to list all accounts
    fn list_accounts(&self) {
        for (account, _) in &self.passwords {
            println!("{}", account);
        }
    }
}

fn greeting() { // Sends a greeting to the user
    println!("Welcome to Rust Password Manager!"); 
} 

fn build_menu() { // Builds the menu options 
    println!("\n-----MENU-----"); 
    println!("1. Add a Password"); 
    println!("2. Get Password"); 
    println!("3. List Accounts"); 
    println!("4. Exit Program");
} 

fn main() {
    let mut password_manager = PasswordManager::new();

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
            let (account, password) = get_account_password();
            password_manager.add_password(account, password);
            println!("\nAdding password");
        } else if choice == 2 {  // Get Password
            println!("\nGetting password");
            print!("Enter account: ");
            io::stdout().flush().unwrap();

            let mut account = String::new();
            io::stdin().read_line(&mut account).unwrap();

            let account = account.trim();
            match password_manager.get_password(account) {
                Some(password) => println!("Password for {}: {}", account, password),
                None => println!("Account not found"),
            }
        } else if choice == 3 {  // List all accounts
            println!("\nAccounts: ");
            password_manager.list_accounts();
        } else if choice == 4 {
            println!("\nClosing program. Goodbye!");
            break;
        }
    }
}

fn get_account_password() -> (String, String) { // Gets a password by the account name
    print!("Enter account: ");
    io::stdout().flush().unwrap();
    let mut account = String::new();
    io::stdin().read_line(&mut account).unwrap();
    
    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    let account = account.trim().to_string();
    let password = password.trim().to_string();

    (account, password)
}