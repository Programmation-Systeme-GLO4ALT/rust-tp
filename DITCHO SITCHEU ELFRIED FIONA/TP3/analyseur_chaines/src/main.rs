// =================================================================
// TP 3 — Analyseur de Chaînes  (Séance 3)
// =================================================================
// Thèmes : références, slices, lifetimes, &str vs String,
//          borrow checker dans un cas réel
// cargo new tp3_analyseur && cp tp3_analyseur.rs tp3_analyseur/src/main.rs
// cd tp3_analyseur && cargo test
// =================================================================

use std::collections::HashMap;

// -----------------------------------------------------------------
//  Structure de résultats
// -----------------------------------------------------------------

#[derive(Debug)]
struct Statistiques {
    nb_mots: usize,
    nb_caracteres: usize,
    nb_phrases: usize,
    mot_le_plus_long: String,          // owned : doit survivre à l'appel
    frequence_chars: [(char, usize); 5], // top 5 caractères alphabétiques
}

// -----------------------------------------------------------------
//  Fonctions d'analyse (toutes basées sur des références)
// -----------------------------------------------------------------

/// Compte les mots séparés par des espaces blancs.
fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

/// Retourne une slice vers le mot le plus long dans le texte.
/// La lifetime 'a garantit que la slice est valide aussi longtemps
/// que le texte source.
fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte
        .split_whitespace()
        .max_by_key(|mot| mot.len())
        .unwrap_or("")
}

/// Vérifie si le texte est un palindrome.
/// Insensible à la casse et ignore les caractères non-alphanumériques
/// (ex : "A man a plan a canal Panama" → true).
fn est_palindrome(texte: &str) -> bool {
    let normalise: String = texte
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();

    let inverse: String = normalise.chars().rev().collect();
    normalise == inverse
}

/// Retourne les N premiers mots du texte sous forme de Vec<&str>.
/// Les slices pointent directement dans 'texte' (zéro copie).
fn premiers_mots<'a>(texte: &'a str, n: usize) -> Vec<&'a str> {
    texte.split_whitespace().take(n).collect()
}

/// Remplace toutes les occurrences de 'de' par 'vers'.
/// Retourne un String owned car la taille peut changer.
fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

/// Calcule des statistiques complètes sur le texte.
fn analyser(texte: &str) -> Statistiques {
    let nb_mots = compter_mots(texte);
    let nb_caracteres = texte.chars().count();

    // Une phrase se termine par '.', '!' ou '?'
    let terminateurs = texte
        .chars()
        .filter(|&c| c == '.' || c == '!' || c == '?')
        .count();
    // Au moins 1 phrase s'il y a du contenu, même sans ponctuation
    let nb_phrases = if nb_mots == 0 { 0 } else { terminateurs.max(1) };

    // Mot le plus long → String owned (doit vivre au-delà de 'texte')
    let mot_le_plus_long = mot_le_plus_long(texte).to_string();

    // Fréquence des caractères alphabétiques (insensible à la casse)
    let mut freq: HashMap<char, usize> = HashMap::new();
    for c in texte.chars() {
        if c.is_alphabetic() {
            *freq.entry(c.to_lowercase().next().unwrap()).or_insert(0) += 1;
        }
    }

    // Tri décroissant par fréquence, puis croissant par caractère
    let mut freq_vec: Vec<(char, usize)> = freq.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    let mut top5 = [(' ', 0usize); 5];
    for (i, &(c, n)) in freq_vec.iter().take(5).enumerate() {
        top5[i] = (c, n);
    }

    Statistiques {
        nb_mots,
        nb_caracteres,
        nb_phrases,
        mot_le_plus_long,
        frequence_chars: top5,
    }
}

// -----------------------------------------------------------------
//  Programme principal
// -----------------------------------------------------------------

fn main() {
    println!("=== TP 3 — Analyseur de Chaînes ===\n");

    let texte = "Le Rust est un langage systeme sans garbage collector. \
                 Il garantit la securite memoire a la compilation! \
                 Apprenez Rust, vous ne le regretterez pas.";

    println!("Texte :\n\"{}\"", texte);
    println!();

    println!("Nombre de mots        : {}", compter_mots(texte));
    println!("Mot le plus long      : \"{}\"", mot_le_plus_long(texte));
    println!("Est palindrome        : {}", est_palindrome(texte));
    println!("\"kayak\" palindrome    : {}", est_palindrome("kayak"));
    println!("\"A man...Panama\"      : {}", est_palindrome("A man a plan a canal Panama"));

    let premiers = premiers_mots(texte, 5);
    println!("5 premiers mots       : {:?}", premiers);

    let remplace = remplacer(texte, "Rust", "Ferris");
    println!("Remplacement Rust→Ferris : \"{}...\"", &remplace[..40]);

    let stats = analyser(texte);
    println!("\n=== Statistiques ===");
    println!("Mots             : {}", stats.nb_mots);
    println!("Caractères       : {}", stats.nb_caracteres);
    println!("Phrases          : {}", stats.nb_phrases);
    println!("Mot le plus long : \"{}\"", stats.mot_le_plus_long);
    println!("Top 5 caractères :");
    for (c, n) in &stats.frequence_chars {
        if *n > 0 {
            println!("  '{}' → {} fois", c, n);
        }
    }
}

// -----------------------------------------------------------------
//  Tests
// -----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compter_mots() {
        assert_eq!(compter_mots("hello world"), 2);
        assert_eq!(compter_mots(""), 0);
        assert_eq!(compter_mots("   espaces   "), 1);
        assert_eq!(compter_mots("un deux trois"), 3);
    }

    #[test]
    fn test_palindrome() {
        assert!(est_palindrome("kayak"));
        assert!(est_palindrome("A man a plan a canal Panama"));
        assert!(est_palindrome("racecar"));
        assert!(!est_palindrome("Rust"));
        assert!(!est_palindrome("hello"));
        assert!(est_palindrome("")); // vide = palindrome trivial
    }

    #[test]
    fn test_mot_le_plus_long() {
        assert_eq!(mot_le_plus_long("chat elephant souris"), "elephant");
        assert_eq!(mot_le_plus_long(""), "");
        assert_eq!(mot_le_plus_long("abc ab a"), "abc");
    }

    #[test]
    fn test_premiers_mots() {
        let mots = premiers_mots("a b c d e f", 3);
        assert_eq!(mots, vec!["a", "b", "c"]);
        // Demander plus que disponible → retourne ce qui existe
        let mots2 = premiers_mots("un deux", 10);
        assert_eq!(mots2.len(), 2);
    }

    #[test]
    fn test_remplacer() {
        assert_eq!(remplacer("Bonjour Rust", "Rust", "monde"), "Bonjour monde");
        assert_eq!(remplacer("aaa", "a", "b"), "bbb");
        assert_eq!(remplacer("rien", "x", "y"), "rien"); // pas de match
    }

    #[test]
    fn test_analyser_stats() {
        let stats = analyser("Bonjour monde!");
        assert_eq!(stats.nb_mots, 2);
        assert_eq!(stats.nb_phrases, 1);
        assert!(!stats.mot_le_plus_long.is_empty());
    }
}
