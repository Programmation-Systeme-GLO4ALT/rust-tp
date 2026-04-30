use std::env;
use std::io::{self, BufRead, Write};

fn calculer(a: f64, op: &str, b: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err(String::from("Division par zéro"))
            } else {
                Ok(a / b)
            }
        }
        _ => Err(format!("Opérateur inconnu : '{}'", op)),
    }
}

fn parse_et_calculer(expr: &str) -> Result<f64, String> {
    let parts: Vec<&str> = expr.trim().split_whitespace().collect();
    if parts.len() != 3 {
        return Err(format!("Format invalide. Attendu: <nombre> <op> <nombre>"));
    }
    let a: f64 = parts[0].parse().map_err(|_| format!("'{}' n'est pas un nombre", parts[0]))?;
    let op = parts[1];
    let b: f64 = parts[2].parse().map_err(|_| format!("'{}' n'est pas un nombre", parts[2]))?;
    calculer(a, op, b)
}

fn mode_interactif() {
    println!("=== Calculatrice interactive ===");
    println!("Format : <nombre> <op> <nombre>  |  Opérateurs : + - * /");
    println!("Tapez 'quitter' pour sortir.\n");

    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut ligne = String::new();
        stdin.lock().read_line(&mut ligne).unwrap();
        let ligne = ligne.trim();

        if ligne == "quitter" || ligne == "q" {
            println!("Au revoir !");
            break;
        }
        if ligne.is_empty() {
            continue;
        }

        match parse_et_calculer(ligne) {
            Ok(r) => println!("= {}", r),
            Err(e) => eprintln!("Erreur : {}", e),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // Mode interactif si aucun argument
        1 => mode_interactif(),
        // Mode CLI : <nombre> <op> <nombre>
        4 => {
            let a: f64 = match args[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("'{}' n'est pas un nombre", args[1]);
                    std::process::exit(1);
                }
            };
            let op = &args[2];
            let b: f64 = match args[3].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("'{}' n'est pas un nombre", args[3]);
                    std::process::exit(1);
                }
            };
            match calculer(a, op, b) {
                Ok(r) => println!("{} {} {} = {}", a, op, b, r),
                Err(e) => {
                    eprintln!("Erreur : {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Usage: {} <nombre> <op> <nombre>", args[0]);
            eprintln!("       {} (mode interactif)", args[0]);
            eprintln!("Opérateurs : + - * /");
            std::process::exit(1);
        }
    }
}
