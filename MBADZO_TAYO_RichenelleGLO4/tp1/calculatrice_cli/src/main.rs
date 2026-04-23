use std::env;

fn calculer(a: f64, op: &str, b: f64) -> Result<f64, String> {
      println!("Hello, Ici MBADZO GLO 4 Bienvenue dans ma calculatrice !");
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

    // ./calculatrice_cli <nombre1> <op> <nombre2>
    if args.len() != 4 {
        eprintln!("Usage: {} <nombre1> <opérateur> <nombre2>", args[0]);
        eprintln!("Opérateurs supportés : + - * /");
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
// on teste comme ceci cargo run -- 10 + 5
