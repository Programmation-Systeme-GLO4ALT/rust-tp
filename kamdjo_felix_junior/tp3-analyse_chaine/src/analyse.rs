// module for string analysis

pub struct Statistiques {
    pub nb_mots: usize,
    pub nb_caracteres: usize,
    pub nb_phrases: usize,
    pub mot_le_plus_long: String,
    pub frequence_chars: [(char, usize); 5],
}

// count words in a text
pub fn compter_mots(texte: &str) -> usize {
    texte
        .split_whitespace()
        .count()
}

// return the longest word
pub fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte
        .split_whitespace()
        .max_by_key(|mot| mot.len())
        .unwrap_or("")
}

// check if text is a palindrome
pub fn est_palindrome(texte: &str) -> bool {
    let s: String = texte
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    s.chars().eq(s.chars().rev())
}

// return first n words as slice references
pub fn premiers_mots(texte: &str, n: usize) -> Vec<&str> {
    texte
        .split_whitespace()
        .take(n)
        .collect()
}

// replace occurrences and return new String
pub fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

// compute statistics
pub fn analyser(texte: &str) -> Statistiques {
    let nb_mots = compter_mots(texte);
    let nb_caracteres = texte.chars().count();
    let nb_phrases = texte.matches(|c| c == '.' || c == '!' || c == '?').count();
    let mot_le_plus_long = mot_le_plus_long(texte).to_string();

    // compute char frequencies
    use std::collections::HashMap;
    let mut freqs = HashMap::new();
    for c in texte.chars().filter(|c| c.is_alphabetic()) {
        *freqs.entry(c).or_insert(0) += 1;
    }
    let mut freqs_vec: Vec<(char, usize)> = freqs.into_iter().collect();
    freqs_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let mut top5 = [(' ', 0); 5];
    for (i, &(c, count)) in freqs_vec.iter().take(5).enumerate() {
        top5[i] = (c, count);
    }

    Statistiques {
        nb_mots,
        nb_caracteres,
        nb_phrases,
        mot_le_plus_long,
        frequence_chars: top5,
    }
}