use std::env;
use std::process;


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
        _ => Err(format!("Opérateur non supporté : {}", op)),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: calculatrice_cli <nombre1> <operateur> <nombre2>");
        process::exit(1);
    }


    let a: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("'{}' n'est pas un nombre", args[1]);
            process::exit(1);
        }
    };


    let op = &args[2];

    let b: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("'{}' n'est pas un nombre", args[3]);
            process::exit(1);
        }
    };

    match calculer(a, op, b) {
        Ok(resultat) => println!("{} {} {} = {}", a, op, b, resultat),
        Err(e) => {
            eprintln!("Erreur : {}", e);
            process::exit(1);
        }
    }
}