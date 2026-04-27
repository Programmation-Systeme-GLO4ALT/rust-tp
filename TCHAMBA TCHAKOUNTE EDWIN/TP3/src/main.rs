// =============================================================
// TP3 — Analyseur de Chaînes
// Tchamba Tchakounte Edwin
//
// Toutes les fonctions utilisent des références (&str / &[T])
// pour ne pas consommer les données passées en argument.
// =============================================================

// Compte les mots dans un texte (séparés par des espaces).
// `split_whitespace` ignore les espaces multiples et les tabs/newlines.
fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

// Retourne le mot le plus long du texte.
// L'annotation 'a indique : le résultat vit aussi longtemps
// que le texte d'entrée (puisque c'est une slice de ce texte).
fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte
        .split_whitespace()
        .max_by_key(|mot| mot.chars().count())
        .unwrap_or("")
}

// Vérifie si le texte est un palindrome.
// On ignore les espaces et la casse pour des phrases comme
// "A man a plan a canal Panama".
fn est_palindrome(texte: &str) -> bool {
    let chars: Vec<char> = texte
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(|c| c.to_lowercase())
        .collect();

    let n = chars.len();
    for i in 0..n / 2 {
        if chars[i] != chars[n - 1 - i] {
            return false;
        }
    }
    true
}

// Retourne les n premiers mots sous forme de slices du texte original.
// Vec<&str> contient des références qui empruntent depuis `texte`.
fn premiers_mots(texte: &str, n: usize) -> Vec<&str> {
    texte.split_whitespace().take(n).collect()
}

// Remplace toutes les occurrences de `de` par `vers`.
// Renvoie une nouvelle String (le texte d'origine n'est pas modifié).
fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

// Structure regroupant les statistiques d'un texte.
#[derive(Debug)]
struct Statistiques {
    nb_mots: usize,
    nb_caracteres: usize,
    nb_phrases: usize,
    mot_le_plus_long: String,
    frequence_chars: [(char, usize); 5],
}

// Analyse complète d'un texte.
fn analyser(texte: &str) -> Statistiques {
    let nb_mots = compter_mots(texte);
    let nb_caracteres = texte.chars().count();
    // Une phrase se termine par . ! ou ?
    let nb_phrases = texte.chars().filter(|c| matches!(c, '.' | '!' | '?')).count();
    let mot_le_plus_long = mot_le_plus_long(texte).to_string();

    // Calcul du top 5 des caractères les plus fréquents
    // (on ignore les espaces, on garde lettres + ponctuation)
    let mut freq: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
    for c in texte.chars().filter(|c| !c.is_whitespace()) {
        *freq.entry(c).or_insert(0) += 1;
    }
    let mut paires: Vec<(char, usize)> = freq.into_iter().collect();
    paires.sort_by(|a, b| b.1.cmp(&a.1)); // tri décroissant par fréquence

    // Remplir le tableau de taille fixe avec le top 5
    let mut top5: [(char, usize); 5] = [(' ', 0); 5];
    for (i, paire) in paires.iter().take(5).enumerate() {
        top5[i] = *paire;
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
    let texte = "Le langage Rust est conçu pour la sûreté mémoire. \
                 Il évite les bugs classiques du C. Il offre des performances \
                 comparables au C++ sans garbage collector !";

    println!("=== Analyse du texte ===");
    println!("Texte : {}\n", texte);

    println!("Nombre de mots : {}", compter_mots(texte));
    println!("Mot le plus long : \"{}\"", mot_le_plus_long(texte));
    println!("Est palindrome ? {}", est_palindrome(texte));

    println!("\n3 premiers mots : {:?}", premiers_mots(texte, 3));
    println!("Remplacer 'Rust' par 'Java' : {}\n",
             remplacer(texte, "Rust", "Java"));

    let stats = analyser(texte);
    println!("=== Statistiques ===");
    println!("{:#?}", stats);

    // Tests rapides de palindromes
    println!("\n=== Tests palindromes ===");
    for s in &["kayak", "Rust", "A man a plan a canal Panama", "Bonjour"] {
        println!("\"{}\" → palindrome ? {}", s, est_palindrome(s));
    }
}

// =============================================================
// Tests unitaires
// `cargo test` les lance automatiquement.
// =============================================================
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
        assert_eq!(mot_le_plus_long("le grand chat"), "grand");
        assert_eq!(mot_le_plus_long("a bb ccc"), "ccc");
    }

    #[test]
    fn test_premiers_mots() {
        assert_eq!(premiers_mots("a b c d", 2), vec!["a", "b"]);
        assert_eq!(premiers_mots("hello", 5), vec!["hello"]);
    }

    #[test]
    fn test_remplacer() {
        assert_eq!(remplacer("hello world", "world", "Rust"), "hello Rust");
        assert_eq!(remplacer("aaa", "a", "b"), "bbb");
    }
}
