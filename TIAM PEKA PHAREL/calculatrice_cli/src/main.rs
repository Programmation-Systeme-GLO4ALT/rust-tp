use std::env;

fn calculer(nombre1: f64, operateur: &str, nombre2: f64) -> Result<f64, String> {
    match operateur {
        "+" => Ok(nombre1 + nombre2),
        "-" => Ok(nombre1 - nombre2),
        "*" => Ok(nombre1 * nombre2),
        "/" => {
            if nombre2 == 0.0 {
                Err(String::from("Division par zéro"))
            } else {
                Ok(nombre1 / nombre2)
            }
        }
        _ => Err(format!("Opérateur inconnu : {}", operateur)),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <nombre1> <operateur> <nombre2>", args[0]);
        eprintln!("Opérateurs : + - * /");
        std::process::exit(1);
    }

    let nombre1: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => { eprintln!("'{}' n'est pas un nombre", args[1]); std::process::exit(1); }
    };
    let operateur = &args[2];
    let nombre2: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => { eprintln!("'{}' n'est pas un nombre", args[3]); std::process::exit(1); }
    };

    match calculer(nombre1, operateur, nombre2) {
        Ok(resultat) => println!("{} {} {} = {}", nombre1, operateur, nombre2, resultat),
        Err(e) => { eprintln!("Erreur : {}", e); std::process::exit(1); }
    }
}
