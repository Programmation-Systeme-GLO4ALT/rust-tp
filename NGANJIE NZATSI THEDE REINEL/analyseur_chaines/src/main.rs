use std::collections::HashMap;

// Compte les mots dans un texte
fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

// Retourne le mot le plus long
fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte
        .split_whitespace()
        .max_by_key(|mot| mot.len())
        .unwrap_or("")
}

// Vérifie si le texte est un palindrome
fn est_palindrome(texte: &str) -> bool {
    let normalise: String = texte
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    normalise.chars().eq(normalise.chars().rev())
}

// Retourne les N premiers mots
fn premiers_mots(texte: &str, n: usize) -> Vec<&str> {
    texte.split_whitespace().take(n).collect()
}

// Remplace les occurrences
fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

struct Statistiques {
    nb_mots: usize,
    nb_caracteres: usize,
    nb_phrases: usize,
    mot_le_plus_long: String,
    frequence_chars: [(char, usize); 5],
}

// Analyse globale
fn analyser(texte: &str) -> Statistiques {
    let nb_mots = compter_mots(texte);
    let nb_caracteres = texte.chars().count();

    let nb_phrases = texte
        .chars()
        .filter(|c| *c == '.' || *c == '!' || *c == '?')
        .count();

    let mot_long = mot_le_plus_long(texte).to_string();

    let mut map: HashMap<char, usize> = HashMap::new();

    for c in texte.chars() {
        if c.is_alphabetic() {
            *map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }

    let mut freq: Vec<(char, usize)> = map.into_iter().collect();

    freq.sort_by(|a, b| b.1.cmp(&a.1));

    let mut top5 = [(' ', 0); 5];

    for (i, (c, n)) in freq.into_iter().take(5).enumerate() {
        top5[i] = (c, n);
    }

    Statistiques {
        nb_mots,
        nb_caracteres,
        nb_phrases,
        mot_le_plus_long: mot_long,
        frequence_chars: top5,
    }
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
fn main() {

    let texte = "A man a plan a canal Panama";

    println!("Texte analysé : {}", texte);
    println!("--------------------------------");

    // 1. Compter les mots
    let nb_mots = compter_mots(texte);
    println!("Nombre de mots : {}", nb_mots);

    // 2. Mot le plus long
    let plus_long = mot_le_plus_long(texte);
    println!("Mot le plus long : {}", plus_long);

    // 3. Vérifier palindrome
    let palindrome = est_palindrome(texte);
    println!("Est un palindrome ? {}", palindrome);

    // 4. Premiers mots
    let premiers = premiers_mots(texte, 3);
    println!("Les 3 premiers mots : {:?}", premiers);

    // 5. Remplacement
    let texte_modifie = remplacer(texte, "Rust", "RUST");
    println!("Texte après remplacement : {}", texte_modifie);

    println!("--------------------------------");

    // 6. Analyse globale
    let stats = analyser(texte);

    println!("Statistiques globales :");
    println!("Nombre de mots : {}", stats.nb_mots);
    println!("Nombre de caractères : {}", stats.nb_caracteres);
    println!("Nombre de phrases : {}", stats.nb_phrases);
    println!("Mot le plus long : {}", stats.mot_le_plus_long);

    println!("Top 5 caractères les plus fréquents :");

    for (c, n) in stats.frequence_chars {
        if c != ' ' {
            println!("{} : {}", c, n);
        }
    }
}