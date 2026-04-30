use std::env;                        


//  La fonction calculer()

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


// BLOC 2 — La fonction main()

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <nombre> <op> <nombre>", args[0]);
        eprintln!("Opérateurs : + - * /");
        std::process::exit(1);
    }

    let a: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("'{}' n'est pas un nombre valide", args[1]);
            std::process::exit(1);
        }
    };

    let op = &args[2];

    let b: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("'{}' n'est pas un nombre valide", args[3]);
            std::process::exit(1);
        }
    };

    match calculer(a, op, b) {
        Ok(resultat) => println!("{} {} {} = {}", a, op, b, resultat),
        Err(e) => {
            eprintln!("Erreur : {}", e);
            std::process::exit(1);
        }
    }
}