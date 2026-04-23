// TP2 — Analyseur de chaînes
// Implémentez ici les fonctions demandées dans l'énoncé TP2

// Compte les mots dans un texte
pub fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

// Retourne le mot le plus long
pub fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte.split_whitespace().max_by_key(|mot| mot.len()).unwrap_or("")
}

// Vérifie si le texte est un palindrome
pub fn est_palindrome(texte: &str) -> bool {
    let texte = texte.to_lowercase();
    let texte: String = texte.chars().filter(|c| c.is_alphabetic()).collect();
    texte.chars().eq(texte.chars().rev())
}

// Retourne les N premiers mots
pub fn premiers_mots(texte: &str, n: usize) -> Vec<&str> {
    texte.split_whitespace().take(n).collect()
}

// Remplace les occurrences (retourne String)
pub fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
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
    #[test]
    fn test_mot_le_plus_long() {
        assert_eq!(mot_le_plus_long("hello world rust"), "world");
    }
    #[test]
    fn test_premiers_mots() {
        assert_eq!(premiers_mots("hello world rust", 2), vec!["hello", "world"]);
    }
    #[test]
    fn test_remplacer() {
        assert_eq!(remplacer("hello world", "world", "Rust"), "hello Rust");
    }
}
