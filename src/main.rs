use std::io;

fn main() {
    println!("Fibonacci Number Generator");

    // Nth Fibonacci number Generator
    nth_fibonacci_number_generator();
}

fn choice() -> u8 {
    loop {
        println!("Choose an option: ");
        println!("1. Generate a Fibonacci number");
        println!("2. Exit");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Failed to read line.");

        match choice.trim().parse() {
            Ok(number) => return number,
            Err(_) => {
                println!("Invalid input. Please enter a number!");
                continue;
            }
        }
    }
}

fn get_fibonacci_number_input() -> u64 {
    loop {
        println!("Enter with the nth Fibonacci number: ");

        let mut number: String = String::new();

        io::stdin().read_line(&mut number).expect("Failed to read line.");

        match number.trim().parse() {
            Ok(number) => return number,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        }
    }
}

fn nth_fibonacci_number(number: u64) -> u64 {
    if number == 0 {
        return 0;
    } else if number == 1 {
        return 1;
    } else {
        return nth_fibonacci_number(number - 1) + nth_fibonacci_number(number - 2)
    }
}

fn nth_fibonacci_number_generator() {
    loop {
        let choice: u8 = choice();

        match choice {
            1 => {
                // Prompt the user for the desired Fibonacci number
                let number = get_fibonacci_number_input();

                // Calculate and display the nth Fibonacci number
                let result = nth_fibonacci_number(number);
                println!("The {number}º Fibonacci number is: {result}");
                println!("");
            }
            2 => {
                println!("Exiting the program!");
                println!("");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a valid option 1 or 2.");
                println!();
            }
        }
    }
}