use std::io;

struct ATM {
    balance: f64,
}

impl ATM {
    fn new(balance: f64) -> Self {
        ATM { balance }
    }

    fn display_menu(&self) {
        println!("\nATM Machine");
        println!("1. Check Balance");
        println!("2. Deposit Money");
        println!("3. Withdraw Money");
        println!("4. Exit");
    }

    fn check_balance(&self) {
        println!("Your current balance is: ${:.2}", self.balance);
    }

    fn deposit_money(&mut self) {
        println!("Enter the amount to deposit: ");
        let amount = self.get_input();

        if amount > 0.0 {
            self.balance += amount;
            println!("${:.2} deposited successfully!", amount);
        } else {
            println!("Invalid amount. Please enter a positive value.");
        }
    }

    fn withdraw_money(&mut self) {
        println!("Enter the amount to withdraw: ");
        let amount = self.get_input();

        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            println!("${:.2} withdrawn successfully!", amount);
        } else if amount > self.balance {
            println!("Insufficient balance.");
        } else {
            println!("Invalid amount. Please enter a positive value.");
        }
    }

    fn get_input(&self) -> f64 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                0.0
            }
        }
    }

    fn run(&mut self) {
        loop {
            self.display_menu();

            println!("Select an option: ");
            let choice = self.get_input() as i32;

            match choice {
                1 => self.check_balance(),
                2 => self.deposit_money(),
                3 => self.withdraw_money(),
                4 => {
                    println!("Thank you for using the ATM. Goodbye!");
                    break;
                }
                _ => println!("Invalid option. Please try again."),
            }
        }
    }
}

fn main() {
    let mut atm = ATM::new(1000.0); // Initial balance: $1000
    atm.run();
}
