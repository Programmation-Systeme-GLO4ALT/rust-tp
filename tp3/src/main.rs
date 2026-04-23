use std::collections::HashMap;

// Compte les mots
fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

// Mot le plus long 
fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte
        .split_whitespace()
        .max_by_key(|mot| mot.len())
        .unwrap_or("")
}

// Vérifier palindrome (ignore espaces + casse)
fn est_palindrome(texte: &str) -> bool {
    let nettoye: String = texte
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();

    nettoye == nettoye.chars().rev().collect::<String>()
}

// Les N premiers mots
fn premiers_mots(texte: &str, n: usize) -> Vec<&str> {
    texte.split_whitespace().take(n).collect()
}

// Remplacement (il retourne un String)
fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

// Structure Statistiques
struct Statistiques {
    nb_mots: usize,
    nb_caracteres: usize,
    nb_phrases: usize,
    mot_le_plus_long: String,
    frequence_chars: [(char, usize); 5],
}

// Analyse complète
fn analyser(texte: &str) -> Statistiques {
    let nb_mots = compter_mots(texte);
    let nb_caracteres = texte.chars().count();
    let nb_phrases = texte.matches(|c| c == '.' || c == '!' || c == '?').count();

    let mot_long = mot_le_plus_long(texte).to_string();

    // fréquence des caractères
    let mut map: HashMap<char, usize> = HashMap::new();

    for c in texte.chars().filter(|c| c.is_alphanumeric()) {
        *map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
    }

    let mut freq: Vec<(char, usize)> = map.into_iter().collect();

    freq.sort_by(|a, b| b.1.cmp(&a.1)); // tri décroissant

    let mut top5 = [(' ', 0); 5];
    for i in 0..5.min(freq.len()) {
        top5[i] = freq[i];
    }

    Statistiques {
        nb_mots,
        nb_caracteres,
        nb_phrases,
        mot_le_plus_long: mot_long,
        frequence_chars: top5,
    }
}

fn main() {
    let texte = "Bonjour tout le monde. Rust est un langage puissant !";

    println!("Mots: {}", compter_mots(texte));
    println!("Plus long: {}", mot_le_plus_long(texte));
    println!("Palindrome: {}", est_palindrome(texte));
    println!("Premiers mots: {:?}", premiers_mots(texte, 3));
    println!("Remplacement: {}", remplacer(texte, "Rust", "C#"));

    let stats = analyser(texte);

    println!("\n=== STATISTIQUES ===");
    println!("Mots: {}", stats.nb_mots);
    println!("Caractères: {}", stats.nb_caracteres);
    println!("Phrases: {}", stats.nb_phrases);
    println!("Mot le plus long: {}", stats.mot_le_plus_long);
    println!("Top 5 caractères: {:?}", stats.frequence_chars);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compter_mots() {
        assert_eq!(compter_mots("hello world"), 2);
        assert_eq!(compter_mots(""), 0);
        assert_eq!(compter_mots("  espaces  "), 1);
    }

    #[test]
    fn test_palindrome() {
        assert!(est_palindrome("kayak"));
        assert!(est_palindrome("A man a plan a canal Panama"));
        assert!(!est_palindrome("Rust"));
    }
}