use std::collections::HashMap;

// Compte les mots dans un texte
fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

// Retourne le mot le plus long
fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte.split_whitespace()
        .max_by_key(|mot| mot.len())
        .unwrap_or("")
}

// Vérifie si le texte est un palindrome
fn est_palindrome(texte: &str) -> bool {
    let nettoye: String = texte
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    
    nettoye.chars().eq(nettoye.chars().rev())
}

// Retourne les N premiers mots
fn premiers_mots(texte: &str, n: usize) -> Vec<&str> {
    texte.split_whitespace().take(n).collect()
}

// Remplace les occurrences (retourne String)
fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

// Statistiques globales
#[derive(Debug, PartialEq)]
struct Statistiques {
    nb_mots: usize,
    nb_caracteres: usize,
    nb_phrases: usize,
    mot_le_plus_long: String,
    frequence_chars: [(char, usize); 5],  // top 5
}

fn analyser(texte: &str) -> Statistiques {
    let nb_mots = compter_mots(texte);
    let nb_caracteres = texte.chars().count();
    let nb_phrases = texte.split(|c| c == '.' || c == '!' || c == '?')
        .filter(|s| !s.trim().is_empty())
        .count();
    let mot_le_plus_long = mot_le_plus_long(texte).to_string();
    
    // Calcul des fréquences des caractères
    let mut freq = HashMap::new();
    for c in texte.chars() {
        if !c.is_whitespace() {
            *freq.entry(c).or_insert(0) += 1;
        }
    }
    
    let mut freq_vec: Vec<(char, usize)> = freq.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
    
    let mut top5: [(char, usize); 5] = [(' ', 0); 5];
    for (i, (c, count)) in freq_vec.iter().take(5).enumerate() {
        top5[i] = (*c, *count);
    }
    
    Statistiques {
        nb_mots,
        nb_caracteres,
        nb_phrases,
        mot_le_plus_long,
        frequence_chars: top5,
    }
}

fn main() {
    let texte = "Rust est un langage moderne! Il est rapide et sûr.";
    let stats = analyser(texte);
    
    println!("--- Analyse de Texte ---");
    println!("Texte : \"{}\"", texte);
    println!("Mots : {}", stats.nb_mots);
    println!("Phrases : {}", stats.nb_phrases);
    println!("Mot le plus long : \"{}\"", stats.mot_le_plus_long);
    println!("Fréquence des caractères (top 5) : {:?}", stats.frequence_chars);
    
    let palindrome = "kayak";
    println!("\n'{}' est un palindrome ? {}", palindrome, est_palindrome(palindrome));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compter_mots() {
        assert_eq!(compter_mots("hello world"), 2);
        assert_eq!(compter_mots(""), 0);
        assert_eq!(compter_mots("  espaces  "), 1);
        assert_eq!(compter_mots("Un   texte   avec   plusieurs   espaces"), 5);
    }

    #[test]
    fn test_mot_le_plus_long() {
        assert_eq!(mot_le_plus_long("hello world"), "hello");
        assert_eq!(mot_le_plus_long("Rust est génial"), "génial");
        assert_eq!(mot_le_plus_long(""), "");
        assert_eq!(mot_le_plus_long("a bb ccc dddd"), "dddd");
    }

    #[test]
    fn test_palindrome() {
        assert!(est_palindrome("kayak"));
        assert!(est_palindrome("A man a plan a canal Panama"));
        assert!(!est_palindrome("Rust"));
        assert!(est_palindrome(""));
        assert!(est_palindrome("a"));
        assert!(est_palindrome("racecar"));
    }

    #[test]
    fn test_premiers_mots() {
        assert_eq!(premiers_mots("un deux trois quatre", 2), vec!["un", "deux"]);
        assert_eq!(premiers_mots("hello world", 5), vec!["hello", "world"]);
        assert_eq!(premiers_mots("", 3), Vec::<&str>::new());
        assert_eq!(premiers_mots("seul", 1), vec!["seul"]);
    }

    #[test]
    fn test_remplacer() {
        assert_eq!(remplacer("hello world", "world", "Rust"), "hello Rust");
        assert_eq!(remplacer("Rust est super", "super", "génial"), "Rust est génial");
        assert_eq!(remplacer("test test test", "test", "ok"), "ok ok ok");
        assert_eq!(remplacer("rien à remplacer", "xxx", "yyy"), "rien à remplacer");
    }

    #[test]
    fn test_analyser() {
        let stats = analyser("Hello world! Comment ça va?");
        
        assert_eq!(stats.nb_mots, 5);
        assert_eq!(stats.nb_caracteres, 24);
        assert_eq!(stats.nb_phrases, 2);
        assert_eq!(stats.mot_le_plus_long, "Comment");
        
        // Vérifier que les caractères sont bien comptés (ignorer espaces)
        assert!(stats.frequence_chars[0].0 != ' ');
    }

    #[test]
    fn test_analyser_texte_vide() {
        let stats = analyser("");
        assert_eq!(stats.nb_mots, 0);
        assert_eq!(stats.nb_caracteres, 0);
        assert_eq!(stats.nb_phrases, 0);
        assert_eq!(stats.mot_le_plus_long, "");
    }

    #[test]
    fn test_analyser_phrase_complexe() {
        let stats = analyser("Rust est un langage moderne! Il est rapide et sûr.");
        
        assert_eq!(stats.nb_mots, 10);
        assert_eq!(stats.nb_phrases, 2);
        assert!(!stats.mot_le_plus_long.is_empty());
    }
}