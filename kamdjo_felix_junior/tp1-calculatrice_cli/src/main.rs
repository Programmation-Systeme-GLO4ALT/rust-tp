use std::env;
use std::io;

fn calculate(a: f64, op: &str, b: f64) -> Option<f64> {
    match op {
        "+" => Some(a + b), // addition
        "-" => Some(a - b), // subtraction
        "*" => Some(a * b), // multiplication
        "/" => {
            if b == 0.0 {
                println!("Error: division by zero"); // handle division by zero
                None
            } else {
                Some(a / b)
            }
        }
        _ => {
            println!("Error: invalid operator"); // invalid operator
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); // collect CLI arguments

    if args.len() == 4 {
        let a: f64 = args[1].parse().expect("Invalid number"); 
        let op = &args[2];
        let b: f64 = args[3].parse().expect("Invalid number"); 

        if let Some(result) = calculate(a, op, b) {
            println!("Result: {:.2}", result); 
        }
        return;
    }

    println!("Interactive mode. Type 'exit' to exit.");

    loop {
        let mut input = String::new();

        println!("Enter expression (e.g. 5 + 3):");
        io::stdin().read_line(&mut input).expect("Failed to read"); 
        let input = input.trim();

        if input == "exit" {
            break; 
        }

        let parts: Vec<&str> = input.split_whitespace().collect(); 

        if parts.len() != 3 {
            println!("Invalid format"); 
            continue;
        }

        let a: f64 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number"); 
                continue;
            }
        };

        let op = parts[1];

        let b: f64 = match parts[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        if let Some(result) = calculate(a, op, b) {
            println!("Result: {:.2}", result); 
    }
}
}