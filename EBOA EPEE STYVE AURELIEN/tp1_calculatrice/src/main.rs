use std::env;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            _ => Err(format!("Operateur inconnu : {}", s)),
        }
    }

    fn symbol(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        }
    }
}

fn calculer(a: f64, op: Operator, b: f64) -> Result<f64, String> {
    match op {
        Operator::Add => Ok(a + b),
        Operator::Subtract => Ok(a - b),
        Operator::Multiply => Ok(a * b),
        Operator::Divide => {
            if b == 0.0 {
                Err(String::from("Division par zero"))
            } else {
                Ok(a / b)
            }
        }
    }
}

fn parser_expression(expression: &str) -> Result<(f64, Operator, f64), String> {
    let parts: Vec<&str> = expression.split_whitespace().collect();

    if parts.len() != 3 {
        return Err(String::from(
            "Format invalide. Exemple attendu : 12 + 5",
        ));
    }

    let a = parts[0]
        .parse::<f64>()
        .map_err(|_| format!("Nombre invalide : {}", parts[0]))?;
    let op = Operator::from_str(parts[1])?;
    let b = parts[2]
        .parse::<f64>()
        .map_err(|_| format!("Nombre invalide : {}", parts[2]))?;

    Ok((a, op, b))
}

fn afficher_resultat(a: f64, op: Operator, b: f64) -> Result<(), String> {
    let resultat = calculer(a, op, b)?;
    println!("{a} {} {b} = {resultat}", op.symbol());
    Ok(())
}

fn mode_interactif() {
    println!("Mode interactif active.");
    println!("Saisis une expression du type : 12 + 5");
    println!("Tape 'quitter' pour sortir.");

    loop {
        print!("> ");
        io::stdout().flush().expect("Impossible de vider stdout");

        let mut entree = String::new();
        if io::stdin().read_line(&mut entree).is_err() {
            eprintln!("Erreur de lecture : impossible de lire l'entree");
            continue;
        }

        let entree = entree.trim();

        if entree.eq_ignore_ascii_case("quitter") {
            println!("Fermeture de la calculatrice.");
            break;
        }

        match parser_expression(entree) {
            Ok((a, op, b)) => {
                if let Err(e) = afficher_resultat(a, op, b) {
                    eprintln!("Erreur : {e}");
                }
            }
            Err(e) => eprintln!("Erreur : {e}"),
        }
    }
}

fn parse_number(arg: &str) -> Result<f64, String> {
    arg.parse::<f64>()
        .map_err(|_| format!("'{}' n'est pas un nombre valide", arg))
}

fn process_arguments(args: &[String]) -> Result<(f64, Operator, f64), String> {
    if args.len() != 3 {
        return Err(String::from("Nombre d'arguments invalide"));
    }

    let a = parse_number(&args[0])?;
    let op = Operator::from_str(&args[1])?;
    let b = parse_number(&args[2])?;

    Ok((a, op, b))
}

fn afficher_aide(programme: &str) {
    eprintln!("Usage :");
    eprintln!("  {programme} <nombre1> <operateur> <nombre2>");
    eprintln!("Exemples :");
    eprintln!("  {programme} 10 + 5");
    eprintln!("  {programme} 12 / 4");
    eprintln!("  {programme}      # lance le mode interactif");
    eprintln!("Operateurs supportes : + - * /");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => mode_interactif(),
        4 => {
            match process_arguments(&args[1..]) {
                Ok((a, op, b)) => {
                    if let Err(e) = afficher_resultat(a, op, b) {
                        eprintln!("Erreur : {e}");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Erreur : {e}");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            afficher_aide(&args[0]);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        assert_eq!(calculer(2.0, Operator::Add, 3.0).unwrap(), 5.0);
    }

    #[test]
    fn division_par_zero() {
        assert!(calculer(8.0, Operator::Divide, 0.0).is_err());
    }

    #[test]
    fn parsing_expression_valide() {
        let (a, op, b) = parser_expression("12 * 3").unwrap();
        assert_eq!(a, 12.0);
        assert!(matches!(op, Operator::Multiply));
        assert_eq!(b, 3.0);
    }

    #[test]
    fn operator_from_str() {
        let op = Operator::from_str("+").unwrap();
        assert!(matches!(op, Operator::Add));
    }

    #[test]
    fn invalid_operator() {
        assert!(Operator::from_str("@").is_err());
    }
}