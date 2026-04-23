// ============================================================
// TP 1 — Setup & CLI Simple : Calculatrice en ligne de commande
// Programmation Système avec Rust — GL4 ENSPD 2025-2026
// ============================================================
//
// Partie C : Calculatrice CLI
//   Usage : cargo run -- <nombre1> <opérateur> <nombre2>
//   Exemple: cargo run -- 10 + 5
//            cargo run -- 15 / 3
//
// Partie D : Mode interactif
//   Usage : cargo run -- --interactif
//   Tapez des expressions comme : 10 + 5
//   Tapez 'quitter' pour sortir
// ============================================================

use std::env;
use std::io::{self, BufRead, Write};

// ------------------------------------------------------------
// Fonction de calcul — retourne Ok(résultat) ou Err(message)
// Supporte +, -, *, /
// ------------------------------------------------------------
fn calculer(a: f64, op: &str, b: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err(String::from("Erreur : division par zéro impossible"))
            } else {
                Ok(a / b)
            }
        }
        _ => Err(format!("Opérateur inconnu : '{}'. Utilisez +, -, *, /", op)),
    }
}

// ------------------------------------------------------------
// Affiche un résultat formaté proprement (entier si pas de décimale)
// ------------------------------------------------------------
fn formater_resultat(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{:.6}", n)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

// ------------------------------------------------------------
// Mode CLI : calcul depuis les arguments de la ligne de commande
// ------------------------------------------------------------
fn mode_cli(args: &[String]) {
    if args.len() != 3 {
        eprintln!("Usage : calculatrice_cli <nombre> <opérateur> <nombre>");
        eprintln!("        calculatrice_cli --interactif");
        eprintln!("Opérateurs : + - * /");
        eprintln!("Exemple    : calculatrice_cli 10.5 * 3");
        std::process::exit(1);
    }

    let a: f64 = match args[0].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Erreur : '{}' n'est pas un nombre valide", args[0]);
            std::process::exit(1);
        }
    };

    let op = &args[1];

    let b: f64 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Erreur : '{}' n'est pas un nombre valide", args[2]);
            std::process::exit(1);
        }
    };

    match calculer(a, op, b) {
        Ok(resultat) => {
            println!(
                "{} {} {} = {}",
                formater_resultat(a),
                op,
                formater_resultat(b),
                formater_resultat(resultat)
            );
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

// ------------------------------------------------------------
// Mode interactif : l'utilisateur saisit des expressions en boucle
// Format attendu : <nombre> <opérateur> <nombre>
// Tapez 'quitter' pour sortir
// ------------------------------------------------------------
fn mode_interactif() {
    let stdin = io::stdin();

    println!("╔══════════════════════════════════════╗");
    println!("║   Calculatrice Interactive — GL4     ║");
    println!("║   Format : <nombre> <op> <nombre>    ║");
    println!("║   Tapez 'quitter' pour sortir        ║");
    println!("╚══════════════════════════════════════╝");

    loop {
        print!("calc> ");
        io::stdout().flush().expect("Erreur flush stdout");

        let mut ligne = String::new();
        match stdin.lock().read_line(&mut ligne) {
            Ok(0) => {
                println!("\nAu revoir !");
                break;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Erreur de lecture : {}", e);
                break;
            }
        }

        let saisie = ligne.trim();

        if saisie.eq_ignore_ascii_case("quitter")
            || saisie.eq_ignore_ascii_case("quit")
            || saisie.eq_ignore_ascii_case("exit")
        {
            println!("Au revoir !");
            break;
        }

        if saisie.is_empty() {
            continue;
        }

        let parties: Vec<&str> = saisie.split_whitespace().collect();
        if parties.len() != 3 {
            eprintln!("  Format attendu : <nombre> <opérateur> <nombre>");
            eprintln!("  Exemple : 10 + 5   ou   3.14 * 2");
            continue;
        }

        let a: f64 = match parties[0].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("  '{}' n'est pas un nombre valide", parties[0]);
                continue;
            }
        };

        let op = parties[1];

        let b: f64 = match parties[2].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("  '{}' n'est pas un nombre valide", parties[2]);
                continue;
            }
        };

        match calculer(a, op, b) {
            Ok(resultat) => println!("  = {}", formater_resultat(resultat)),
            Err(e) => eprintln!("  {}", e),
        }
    }
}

// ------------------------------------------------------------
// Point d'entrée principal
// ------------------------------------------------------------
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args[0] == "--interactif" || args[0] == "-i" {
        mode_interactif();
    } else {
        mode_cli(&args);
    }
}

// ============================================================
// Tests unitaires — cargo test
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculer(10.0, "+", 5.0), Ok(15.0));
    }

    #[test]
    fn test_soustraction() {
        assert_eq!(calculer(10.0, "-", 3.0), Ok(7.0));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculer(4.0, "*", 3.0), Ok(12.0));
    }

    #[test]
    fn test_division() {
        assert_eq!(calculer(10.0, "/", 2.0), Ok(5.0));
    }

    #[test]
    fn test_division_par_zero() {
        assert!(calculer(5.0, "/", 0.0).is_err());
    }

    #[test]
    fn test_operateur_inconnu() {
        assert!(calculer(5.0, "%", 2.0).is_err());
    }

    #[test]
    fn test_nombres_negatifs() {
        assert_eq!(calculer(-5.0, "+", 3.0), Ok(-2.0));
    }

    #[test]
    fn test_formater_entier() {
        assert_eq!(formater_resultat(5.0), "5");
    }

    #[test]
    fn test_formater_reel() {
        assert_eq!(formater_resultat(3.14), "3.14");
    }
}
