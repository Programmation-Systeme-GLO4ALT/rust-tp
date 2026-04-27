use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <nombre1> <opérateur> <nombre2>", args[0]);
        eprintln!("Opérateurs supportés : + - * /");
        return;
    }

    let a: f64 = args[1].parse().expect("Le 1er argument doit être un nombre");
    let op: &str = &args[2];
    let b: f64 = args[3].parse().expect("Le 3ème argument doit être un nombre");

    let resultat = match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => {
            eprintln!("Opérateur inconnu : {}", op);
            return;
        }
    };

    println!("{} {} {} = {}", a, op, b, resultat);
}
