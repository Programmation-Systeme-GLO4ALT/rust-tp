use std::env;
use std::io::{self, BufRead, Write};

fn calculer(a: f64, op: &str, b: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err(String::from("Division par zéro impossible"))
            } else {
                Ok(a / b)
            }
        }
        _ => Err(format!("Opérateur inconnu : '{}'. Utilisez +, -, *, /", op)),
    }
}

fn mode_interactif() {
    let stdin = io::stdin();
    println!("=== Calculatrice Interactive ===");
    println!("Format : <nombre> <opérateur> <nombre>");
    println!("Opérateurs : + - * /");
    println!("Tapez 'quitter' pour sortir\n");

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

        let parts: Vec<&str> = ligne.split_whitespace().collect();
        if parts.len() != 3 {
            eprintln!("Format invalide. Exemple : 10 + 5");
            continue;
        }

        let a: f64 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => { eprintln!("'{}' n'est pas un nombre valide", parts[0]); continue; }
        };
        let op = parts[1];
        let b: f64 = match parts[2].parse() {
            Ok(n) => n,
            Err(_) => { eprintln!("'{}' n'est pas un nombre valide", parts[2]); continue; }
        };

        match calculer(a, op, b) {
            Ok(r)  => println!("  {} {} {} = {}", a, op, b, r),
            Err(e) => eprintln!("  Erreur : {}", e),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        mode_interactif();
        return;
    }

    if args.len() != 4 {
        eprintln!("Usage: {} <nombre> <op> <nombre>", args[0]);
        eprintln!("Opérateurs : + - * /");
        eprintln!("Sans arguments : mode interactif");
        std::process::exit(1);
    }

    let a: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => { eprintln!("'{}' n'est pas un nombre valide", args[1]); std::process::exit(1); }
    };
    let op = &args[2];
    let b: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => { eprintln!("'{}' n'est pas un nombre valide", args[3]); std::process::exit(1); }
    };

    match calculer(a, op, b) {
        Ok(r)  => println!("{} {} {} = {}", a, op, b, r),
        Err(e) => { eprintln!("Erreur : {}", e); std::process::exit(1); }
    }
}

#[cfg(test)]
mod tests_tp1 {
    use super::calculer;

    #[test]
    fn test_addition()         { assert_eq!(calculer(3.0, "+", 4.0), Ok(7.0)); }
    #[test]
    fn test_soustraction()     { assert_eq!(calculer(10.0, "-", 4.0), Ok(6.0)); }
    #[test]
    fn test_multiplication()   { assert_eq!(calculer(3.0, "*", 4.0), Ok(12.0)); }
    #[test]
    fn test_division()         { assert_eq!(calculer(10.0, "/", 2.0), Ok(5.0)); }
    #[test]
    fn test_division_par_zero(){ assert!(calculer(10.0, "/", 0.0).is_err()); }
    #[test]
    fn test_op_inconnu()       { assert!(calculer(1.0, "%", 1.0).is_err()); }
}
