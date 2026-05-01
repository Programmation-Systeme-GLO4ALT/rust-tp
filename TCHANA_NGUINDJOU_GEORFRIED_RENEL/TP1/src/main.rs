// ============================================================
// TP 1 — Calculatrice CLI + Mode Interactif
// Séance 1 : Introduction & Environnement
// ============================================================
// Utilisation :
//   cargo run -- 10 + 5        => mode CLI (args)
//   cargo run                  => mode interactif
// ============================================================

use std::env;
use std::io::{self, BufRead};
use std::process;

/// Effectue le calcul selon l'opérateur fourni.
/// Retourne Ok(résultat) ou Err(message d'erreur).
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
        _ => Err(format!("Opérateur '{}' non supporté. Utilisez + - * /", op)),
    }
}

/// Affiche le résultat formaté (sans .0 inutile pour les entiers).
fn afficher_resultat(a: f64, op: &str, b: f64, resultat: f64) {
    if resultat.fract() == 0.0 && resultat.abs() < 1e15 {
        println!("{} {} {} = {}", a, op, b, resultat as i64);
    } else {
        println!("{} {} {} = {:.6}", a, op, b, resultat);
    }
}

/// Mode CLI : lit les arguments et effectue un calcul unique.
fn mode_cli(args: &[String]) {
    if args.len() != 4 {
        eprintln!("Usage : {} <nombre1> <opérateur> <nombre2>", args[0]);
        eprintln!("Exemple : {} 10 + 5", args[0]);
        eprintln!("Opérateurs supportés : + - * /");
        process::exit(1);
    }

    let a: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Erreur : '{}' n'est pas un nombre valide", args[1]);
            process::exit(1);
        }
    };

    let op: &str = &args[2];

    let b: f64 = match args[3].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Erreur : '{}' n'est pas un nombre valide", args[3]);
            process::exit(1);
        }
    };

    match calculer(a, op, b) {
        Ok(resultat) => afficher_resultat(a, op, b, resultat),
        Err(message) => {
            eprintln!("{}", message);
            process::exit(1);
        }
    }
}

/// Mode interactif : boucle de lecture depuis stdin.
fn mode_interactif() {
    println!("=== Calculatrice Interactive Rust ===");
    println!("Format  : <nombre> <opérateur> <nombre>");
    println!("Exemple : 10 + 5  |  3.14 * 2  |  100 / 4");
    println!("Tapez 'quitter' ou 'q' pour sortir.");
    println!();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let saisie = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let saisie = saisie.trim().to_string();

        // Conditions de sortie
        if saisie == "quitter" || saisie == "q" || saisie == "exit" {
            println!("Au revoir !");
            break;
        }

        if saisie.is_empty() {
            continue;
        }

        // Découpage en tokens (gère les espaces multiples)
        let tokens: Vec<&str> = saisie.split_whitespace().collect();

        if tokens.len() != 3 {
            eprintln!("Format invalide. Exemple : 10 + 5");
            continue;
        }

        let a: f64 = match tokens[0].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Nombre invalide : '{}'", tokens[0]);
                continue;
            }
        };

        let b: f64 = match tokens[2].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Nombre invalide : '{}'", tokens[2]);
                continue;
            }
        };

        match calculer(a, tokens[1], b) {
            Ok(res) => afficher_resultat(a, tokens[1], b, res),
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Si des arguments sont fournis → mode CLI, sinon → mode interactif
    if args.len() > 1 {
        mode_cli(&args);
    } else {
        mode_interactif();
    }
}

// ============================================================
// Tests unitaires
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculer(3.0, "+", 4.0), Ok(7.0));
    }

    #[test]
    fn test_soustraction() {
        assert_eq!(calculer(10.0, "-", 3.0), Ok(7.0));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculer(6.0, "*", 7.0), Ok(42.0));
    }

    #[test]
    fn test_division() {
        assert_eq!(calculer(10.0, "/", 2.0), Ok(5.0));
    }

    #[test]
    fn test_division_par_zero() {
        assert!(calculer(10.0, "/", 0.0).is_err());
    }

    #[test]
    fn test_operateur_inconnu() {
        assert!(calculer(1.0, "%", 2.0).is_err());
    }
}
