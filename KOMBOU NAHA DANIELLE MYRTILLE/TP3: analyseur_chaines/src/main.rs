fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte.split_whitespace()
        .max_by_key(|mot| {
            mot.chars().filter(|c| c.is_alphabetic()).count()
        })
        .unwrap_or("")
}

fn est_palindrome(texte: &str) -> bool {
    let propre: String = texte.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    let inverse: String = propre.chars().rev().collect();
    propre == inverse
}

fn premiers_mots<'a>(texte: &'a str, n: usize) -> Vec<&'a str> {
    texte.split_whitespace().take(n).collect()
}

fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

struct Statistiques {
    nb_mots: usize,
    nb_caracteres: usize,
    nb_phrases: usize,
    mot_le_plus_long: String,
}

fn analyser(texte: &str) -> Statistiques {
    Statistiques {
        nb_mots: compter_mots(texte),
        nb_caracteres: texte.chars().count(),
        nb_phrases: texte.chars()
            .filter(|&c| c == '.' || c == '!' || c == '?')
            .count(),
        mot_le_plus_long: String::from(mot_le_plus_long(texte)),
    }
}

fn main() {
    let texte = "Rust est un langage système rapide et sûr. \
                 Il garantit la sécurité mémoire! \
                 Est-ce difficile à apprendre?";

    println!("=== Analyseur de Texte ===\n");
    println!("Texte : {}\n", texte);

    // Test compter_mots
    println!("Nombre de mots     : {}", compter_mots(texte));

    // Test mot_le_plus_long
    println!("Mot le plus long   : {}", mot_le_plus_long(texte));

    // Test palindrome
    println!("Est palindrome     : {}", est_palindrome(texte));
    println!("'kayak' palindrome : {}", est_palindrome("kayak"));
    println!("'A man a plan a canal Panama' : {}",
             est_palindrome("A man a plan a canal Panama"));

    // Test premiers_mots
    let premiers = premiers_mots(texte, 4);
    println!("4 premiers mots    : {:?}", premiers);

    // Test remplacer
    let nouveau = remplacer(texte, "Rust", "Python");
    println!("Après remplacement : {}", &nouveau[..40]);

    // Test analyser
    let stats = analyser(texte);
    println!("\n=== Statistiques ===");
    println!("Mots               : {}", stats.nb_mots);
    println!("Caractères         : {}", stats.nb_caracteres);
    println!("Phrases            : {}", stats.nb_phrases);
    println!("Mot le plus long   : {}", stats.mot_le_plus_long);
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