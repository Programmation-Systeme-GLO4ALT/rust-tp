use std::env;

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

    if args.len() != 4 {
        println!("Usage: {} <nombre> <op> <nombre>", args[0]);
        return;
    }

    let a: f64 = args[1].parse().expect("Nombre invalide");
    let op = &args[2];
    let b: f64 = args[3].parse().expect("Nombre invalide");

    match calculer(a, op, b) {
        Ok(resultat) => println!("{} {} {} = {}", a, op, b, resultat),
        Err(e) => println!("Erreur : {}", e),
    }
}