use std::env;
use std::io::{self, Write};


fn lire_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn mode_interactif() {
    println!("🧮 Mode interactif");

    let a: f64 = lire_input("Premier nombre: ")
        .parse()
        .expect("Entrée invalide");

    let op = lire_input("Opérateur (+ - * /): ");

    let b: f64 = lire_input("Deuxième nombre: ")
        .parse()
        .expect("Entrée invalide");

    match calculer(a, &op, b) {
        Ok(resultat) => println!("{} {} {} = {}", a, op, b, resultat),
        Err(e) => eprintln!("Erreur : {}", e),
    }
}


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
        _ => Err(format!("Opérateur inconnu : {}", op)),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // 👉 Mode interactif
    if args.len() == 2 && args[1] == "--interactive" {
        mode_interactif();
        return;
    }



    // mode normal
    if args.len() != 4 {
        eprintln!("Usage: {} <nombre> <op> <nombre>", args[0]);
        eprintln!("Opérateurs : + - * /");
        std::process::exit(1);
    }

    let a: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => { eprintln!("'{}' n'est pas un nombre", args[1]); std::process::exit(1); }
    };
    let op = &args[2];
    let b: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => { eprintln!("'{}' n'est pas un nombre", args[3]); std::process::exit(1); }
    };

    match calculer(a, op, b) {
        Ok(resultat) => println!("{} {} {} = {}", a, op, b, resultat),
        Err(e) => { eprintln!("Erreur : {}", e); std::process::exit(1); }
    }
}
