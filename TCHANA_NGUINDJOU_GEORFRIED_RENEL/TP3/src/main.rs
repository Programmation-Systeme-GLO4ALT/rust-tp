// ============================================================
// TP 3 — Analyseur de Chaînes
// Séance 3 : Ownership & Borrowing — Partie II
// ============================================================
// Concepts couverts :
//   - Références immuables (&str, &[T])
//   - Lifetimes explicites ('a)
//   - Slices de chaînes
//   - Borrow checker dans un cas réel
// ============================================================

use std::collections::HashMap;

// ============================================================
// Fonctions de base
// ============================================================

/// Compte le nombre de mots dans un texte.
/// split_whitespace() gère les espaces multiples, tabulations et sauts de ligne.
pub fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

/// Retourne le mot le plus long du texte.
/// La lifetime 'a garantit que la slice retournée reste valide
/// tant que 'texte' est valide — zéro copie.
pub fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte
        .split_whitespace()
        .max_by_key(|mot| mot.len())
        .unwrap_or("")
}

/// Vérifie si le texte est un palindrome.
/// Ignore la casse et tous les caractères non-alphanumériques.
/// Exemple : "A man a plan a canal Panama" → true
pub fn est_palindrome(texte: &str) -> bool {
    // Normalisation : garder seulement les alphanum, tout en minuscule
    let normalise: String = texte
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();

    // Comparaison avec la version renversée
    let inverse: String = normalise.chars().rev().collect();
    normalise == inverse
}

/// Retourne les N premiers mots sous forme de Vec de slices.
/// Les &str dans le Vec pointent vers les données originales : zéro allocation.
pub fn premiers_mots<'a>(texte: &'a str, n: usize) -> Vec<&'a str> {
    texte.split_whitespace().take(n).collect()
}

/// Remplace toutes les occurrences de 'de' par 'vers'.
/// Retourne une nouvelle String allouée.
pub fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

// ============================================================
// Structure de statistiques
// ============================================================

#[derive(Debug)]
pub struct Statistiques {
    pub nb_mots: usize,
    pub nb_caracteres: usize,
    pub nb_phrases: usize,
    pub mot_le_plus_long: String,
    pub frequence_chars: Vec<(char, usize)>, // Top 5 caractères les plus fréquents
}

/// Calcule toutes les statistiques d'un texte.
pub fn analyser(texte: &str) -> Statistiques {
    let nb_mots = compter_mots(texte);

    // .chars().count() compte les codepoints Unicode (correct pour l'UTF-8)
    // .len() compterait les octets — différent pour les accents/emojis
    let nb_caracteres = texte.chars().count();

    // Heuristique : compte les . ! ? comme fins de phrases
    let nb_phrases_ponct = texte
        .chars()
        .filter(|&c| c == '.' || c == '!' || c == '?')
        .count();
    // Au minimum 1 phrase si du texte existe
    let nb_phrases = if nb_phrases_ponct == 0 && nb_mots > 0 {
        1
    } else {
        nb_phrases_ponct
    };

    // mot_le_plus_long retourne &str → on clone pour posséder la String
    let long = mot_le_plus_long(texte).to_string();

    // Fréquence des caractères alphabétiques (insensible à la casse)
    let mut freq: HashMap<char, usize> = HashMap::new();
    for c in texte.chars() {
        if c.is_alphabetic() {
            if let Some(lc) = c.to_lowercase().next() {
                *freq.entry(lc).or_insert(0) += 1;
            }
        }
    }

    // Tri par fréquence décroissante, puis garder le top 5
    let mut freq_vec: Vec<(char, usize)> = freq.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    freq_vec.truncate(5);

    Statistiques {
        nb_mots,
        nb_caracteres,
        nb_phrases,
        mot_le_plus_long: long,
        frequence_chars: freq_vec,
    }
}

// ============================================================
// Main
// ============================================================

fn main() {
    let texte = "Rust est un langage de programmation système. \
                 Il garantit la sécurité mémoire sans garbage collector. \
                 Rust est rapide et fiable!";

    println!("=== Analyseur de Chaînes Rust (TP 3) ===\n");
    println!("Texte analysé :");
    println!("  \"{}\"\n", texte);

    // Fonctions de base
    println!("Nombre de mots      : {}", compter_mots(texte));
    println!("Mot le plus long    : \"{}\"", mot_le_plus_long(texte));
    println!("Est un palindrome ? : {}", est_palindrome(texte));
    println!(
        "5 premiers mots     : {:?}",
        premiers_mots(texte, 5)
    );
    println!(
        "Remplacement        : \"{}\"",
        remplacer(texte, "Rust", "C++")
    );

    // Tests de palindrome
    println!("\n--- Tests palindromes ---");
    let exemples = [
        "kayak",
        "A man a plan a canal Panama",
        "racecar",
        "Rust",
        "Bonjour",
    ];
    for ex in &exemples {
        println!("  \"{}\" → {}", ex, est_palindrome(ex));
    }

    // Statistiques globales
    println!("\n--- Statistiques globales ---");
    let stats = analyser(texte);
    println!("Mots              : {}", stats.nb_mots);
    println!("Caractères        : {}", stats.nb_caracteres);
    println!("Phrases estimées  : {}", stats.nb_phrases);
    println!("Mot le plus long  : \"{}\"", stats.mot_le_plus_long);
    println!("Top 5 caractères  :");
    for (c, n) in &stats.frequence_chars {
        println!("  '{}' → {} fois", c, n);
    }
}

// ============================================================
// Tests unitaires (cargo test)
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    // ── compter_mots ─────────────────────────────────────────
    #[test]
    fn test_compter_mots_normal() {
        assert_eq!(compter_mots("hello world"), 2);
    }

    #[test]
    fn test_compter_mots_vide() {
        assert_eq!(compter_mots(""), 0);
    }

    #[test]
    fn test_compter_mots_espaces() {
        assert_eq!(compter_mots("  espaces  "), 1);
    }

    #[test]
    fn test_compter_mots_multiligne() {
        assert_eq!(compter_mots("un\ndeux\ntrois"), 3);
    }

    // ── mot_le_plus_long ─────────────────────────────────────
    #[test]
    fn test_mot_le_plus_long_normal() {
        assert_eq!(mot_le_plus_long("Rust est formidable"), "formidable");
    }

    #[test]
    fn test_mot_le_plus_long_vide() {
        assert_eq!(mot_le_plus_long(""), "");
    }

    #[test]
    fn test_mot_le_plus_long_egal() {
        // max_by_key prend le dernier en cas d'égalité
        let resultat = mot_le_plus_long("abc def");
        assert_eq!(resultat.len(), 3);
    }

    // ── est_palindrome ───────────────────────────────────────
    #[test]
    fn test_palindrome_simple() {
        assert!(est_palindrome("kayak"));
        assert!(est_palindrome("racecar"));
    }

    #[test]
    fn test_palindrome_phrase() {
        assert!(est_palindrome("A man a plan a canal Panama"));
    }

    #[test]
    fn test_palindrome_negatif() {
        assert!(!est_palindrome("Rust"));
        assert!(!est_palindrome("Bonjour"));
    }

    #[test]
    fn test_palindrome_vide() {
        assert!(est_palindrome("")); // vide = palindrome trivial
    }

    #[test]
    fn test_palindrome_un_char() {
        assert!(est_palindrome("a"));
    }

    // ── premiers_mots ────────────────────────────────────────
    #[test]
    fn test_premiers_mots_normal() {
        let res = premiers_mots("un deux trois quatre", 3);
        assert_eq!(res, vec!["un", "deux", "trois"]);
    }

    #[test]
    fn test_premiers_mots_depasse() {
        // n > nombre de mots : retourne tout
        let res = premiers_mots("un deux", 10);
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_premiers_mots_vide() {
        assert!(premiers_mots("", 3).is_empty());
    }

    // ── remplacer ────────────────────────────────────────────
    #[test]
    fn test_remplacer_normal() {
        assert_eq!(remplacer("Hello Rust", "Rust", "World"), "Hello World");
    }

    #[test]
    fn test_remplacer_toutes_occurrences() {
        assert_eq!(remplacer("aaa", "a", "b"), "bbb");
    }

    #[test]
    fn test_remplacer_absent() {
        assert_eq!(remplacer("abc", "x", "y"), "abc");
    }

    // ── analyser ─────────────────────────────────────────────
    #[test]
    fn test_analyser_mots() {
        let stats = analyser("Bonjour le monde");
        assert_eq!(stats.nb_mots, 3);
    }

    #[test]
    fn test_analyser_phrases() {
        let stats = analyser("Phrase un. Phrase deux! Trois?");
        assert_eq!(stats.nb_phrases, 3);
    }

    #[test]
    fn test_analyser_vide() {
        let stats = analyser("");
        assert_eq!(stats.nb_mots, 0);
        assert_eq!(stats.nb_caracteres, 0);
        assert_eq!(stats.nb_phrases, 0);
    }
}
