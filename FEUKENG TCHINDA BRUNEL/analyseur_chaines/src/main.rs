use std::io::{self, Write};
use std::collections::HashMap;

// Fonction pour compter les mots
fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

// Fonction pour trouver le mot le plus long
fn mot_le_plus_long(texte: &str) -> &str {
    texte.split_whitespace()
        .max_by_key(|mot| mot.len())
        .unwrap_or("")
}

// Fonction pour vérifier si une chaîne est un palindrome
fn est_palindrome(texte: &str) -> bool {
    let nettoye: String = texte
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let inverse: String = nettoye.chars().rev().collect();
    nettoye == inverse
}

// Fonction pour compter les phrases
fn compter_phrases(texte: &str) -> usize {
    texte.split(|c| c == '.' || c == '!' || c == '?')
        .filter(|s| !s.trim().is_empty())
        .count()
}

// Fonction pour compter la fréquence des caractères
fn frequence_caracteres(texte: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for c in texte.chars() {
        if !c.is_whitespace() {
            *freq.entry(c).or_insert(0) += 1;
        }
    }
    freq
}

// Structure pour les statistiques
struct Statistiques {
    nombre_mots: usize,
    nombre_phrases: usize,
    mot_le_plus_long: String,
    longueur_moyenne: f64,
    est_palindrome: bool,
}

impl Statistiques {
    fn analyser(texte: &str) -> Statistiques {
        let mots: Vec<&str> = texte.split_whitespace().collect();
        let total_lettres: usize = mots.iter().map(|m| m.len()).sum();
        let longueur_moyenne = if mots.is_empty() {
            0.0
        } else {
            total_lettres as f64 / mots.len() as f64
        };

        Statistiques {
            nombre_mots: mots.len(),
            nombre_phrases: compter_phrases(texte),
            mot_le_plus_long: mot_le_plus_long(texte).to_string(),
            longueur_moyenne,
            est_palindrome: est_palindrome(texte),
        }
    }

    fn afficher(&self) {
        println!("\n=== STATISTIQUES DU TEXTE ===");
        println!(" Nombre de mots: {}", self.nombre_mots);
        println!(" Nombre de phrases: {}", self.nombre_phrases);
        println!(" Mot le plus long: '{}'", self.mot_le_plus_long);
        println!(" Longueur moyenne des mots: {:.2}", self.longueur_moyenne);
        println!(" Palindrome: {}", if self.est_palindrome { "OUI ✓" } else { "NON ✗" });
        println!("==============================\n");
    }
}

fn main() {
    println!("\n=== ANALYSEUR DE TEXTES ===\n");

    loop {
        println!("Que voulez-vous faire ?");
        println!("1. Analyser un texte");
        println!("2. Compter les mots d'une phrase");
        println!("3. Vérifier si un palindrome");
        println!("4. Analyser les caractères");
        println!("5. Quitter");
        print!("Choix: ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();
        let choix = choix.trim();

        match choix {
            "1" => {
                print!("\nEntrez votre texte: ");
                io::stdout().flush().unwrap();
                let mut texte = String::new();
                io::stdin().read_line(&mut texte).unwrap();
                let texte = texte.trim();

                let stats = Statistiques::analyser(texte);
                stats.afficher();

                // Afficher la fréquence des caractères
                println!(" Fréquence des caractères:");
                let freq = frequence_caracteres(texte);
                for (c, count) in freq.iter().take(10) {
                    println!("   '{}': {}", c, count);
                }
                if freq.len() > 10 {
                    println!("   ... et {} autres caractères", freq.len() - 10);
                }
            }
            "2" => {
                print!("\nEntrez une phrase: ");
                io::stdout().flush().unwrap();
                let mut phrase = String::new();
                io::stdin().read_line(&mut phrase).unwrap();
                let phrase = phrase.trim();

                let mots = compter_mots(phrase);
                println!(" Nombre de mots: {}", mots);
            }
            "3" => {
                print!("\nEntrez un mot ou une phrase: ");
                io::stdout().flush().unwrap();
                let mut mot = String::new();
                io::stdin().read_line(&mut mot).unwrap();
                let mot = mot.trim();

                if est_palindrome(mot) {
                    println!(" '{}' est un palindrome !", mot);
                } else {
                    println!(" '{}' n'est pas un palindrome", mot);
                }
            }
            "4" => {
                print!("\nEntrez un texte: ");
                io::stdout().flush().unwrap();
                let mut texte = String::new();
                io::stdin().read_line(&mut texte).unwrap();
                let texte = texte.trim();

                let freq = frequence_caracteres(texte);
                println!("\n Fréquence des caractères:");
                let mut pairs: Vec<_> = freq.into_iter().collect();
                pairs.sort_by(|a, b| b.1.cmp(&a.1));

                for (c, count) in pairs {
                    println!("   '{}': {} fois", c, count);
                }
            }
            "5" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide !"),
        }
        println!();
    }
}