use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Mode interactif si pas d'arguments
    if args.len() == 1 {
        mode_interactif();
    } else {
        // Mode ligne de commande
        let operation = &args[1];
        if args.len() < 4 {
            println!("Usage: calculatrice_cli <opération> <a> <b>");
            println!("Opérations: add, sub, mul, div");
            return;
        }

        let a: f64 = args[2].parse().unwrap();
        let b: f64 = args[3].parse().unwrap();

        match operation.as_str() {
            "add" => println!("{} + {} = {}", a, b, a + b),
            "sub" => println!("{} - {} = {}", a, b, a - b),
            "mul" => println!("{} × {} = {}", a, b, a * b),
            "div" => {
                if b == 0.0 {
                    println!("Erreur: Division par zéro !");
                } else {
                    println!("{} ÷ {} = {}", a, b, a / b);
                }
            }
            _ => println!("Opération inconnue. Utilisez: add, sub, mul, div"),
        }
    }
}

fn mode_interactif() {
    println!("\n=== Calculatrice Interactive ====");
    println!("Commandes: add <a> <b>, sub <a> <b>, mul <a> <b>, div <a> <b>, quit");
    println!("================================\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        let mots: Vec<&str> = input.split_whitespace().collect();

        if mots.is_empty() {
            continue;
        }

        match mots[0] {
            "quit" => {
                println!("Au revoir !");
                break;
            }
            "add" | "sub" | "mul" | "div" => {
                if mots.len() != 3 {
                    println!("Erreur: Il faut 2 nombres. Exemple: add 5 3");
                    continue;
                }

                let a: f64 = match mots[1].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Erreur: '{}' n'est pas un nombre", mots[1]);
                        continue;
                    }
                };

                let b: f64 = match mots[2].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Erreur: '{}' n'est pas un nombre", mots[2]);
                        continue;
                    }
                };

                match mots[0] {
                    "add" => println!("{} + {} = {}", a, b, a + b),
                    "sub" => println!("{} - {} = {}", a, b, a - b),
                    "mul" => println!("{} × {} = {}", a, b, a * b),
                    "div" => {
                        if b == 0.0 {
                            println!("Erreur: Division par zéro !");
                        } else {
                            println!("{} ÷ {} = {}", a, b, a / b);
                        }
                    }
                    _ => {}
                }
            }
            _ => println!("Commande inconnue. Utilisez: add, sub, mul, div, quit"),
        }
    }
}