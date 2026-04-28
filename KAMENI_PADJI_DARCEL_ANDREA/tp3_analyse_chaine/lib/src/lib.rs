
use std::collections::HashMap;

pub fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

pub fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte.split_whitespace().max_by_key(|mot| mot.len()).unwrap_or("")
}

pub fn est_palindrome(texte: &str) -> bool {
    let texte = texte.to_lowercase();
    let texte: String = texte.chars().filter(|c| c.is_alphabetic()).collect();
    texte.chars().eq(texte.chars().rev())
}

pub fn premiers_mots(texte: &str, n: usize) -> Vec<&str> {
    texte.split_whitespace().take(n).collect()
}

pub fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

#[derive(Debug, PartialEq)]
pub struct Statistiques {
    pub nb_mots: usize,
    pub nb_caracteres: usize,
    pub nb_phrases: usize,
    pub mot_le_plus_long: String,
    pub frequence_chars: [(char, usize); 5],
}

pub fn analyser(texte: &str) -> Statistiques {
    let nb_mots = compter_mots(texte);
    let nb_caracteres = texte.chars().count();
    let nb_phrases = texte.split(|c| c == '.' || c == '!' || c == '?').filter(|s| !s.trim().is_empty()).count();
    let mot_le_plus_long = mot_le_plus_long(texte).to_string();

    let mut frequence: HashMap<char, usize> = HashMap::new();
    for c in texte.chars().filter(|c| c.is_alphabetic()) {
        *frequence.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
    }
    let mut frequence_vec: Vec<_> = frequence.into_iter().collect();
    frequence_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let top_5 = frequence_vec.into_iter().take(5).collect::<Vec<_>>();
    let mut frequence_chars = [(' ', 0); 5];
    for (i, &(c, count)) in top_5.iter().enumerate() {
        frequence_chars[i] = (c, count);
    }

    Statistiques {
        nb_mots,
        nb_caracteres,
        nb_phrases,
        mot_le_plus_long,
        frequence_chars,
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
