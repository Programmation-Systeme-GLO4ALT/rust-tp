// ============================================================
// TP 3 — Analyseur de Chaînes (Références & Lifetimes)
// Programmation Système avec Rust — GL4 ENSPD 2025-2026
// ============================================================
//
// Toutes les fonctions travaillent sur des références &str.
// Aucun String n'est cloné inutilement.
// Les lifetimes sont annotées là où le compilateur en a besoin.
// ============================================================

use std::collections::HashMap;

// ------------------------------------------------------------
// Structure Statistiques : résumé d'un texte analysé
// ------------------------------------------------------------
#[derive(Debug)]
struct Statistiques {
    nb_mots: usize,
    nb_caracteres: usize,  // hors espaces
    nb_phrases: usize,
    mot_le_plus_long: String,
    frequence_chars: [(char, usize); 5], // top 5 caractères les plus fréquents
}

// ------------------------------------------------------------
// Compte le nombre de mots dans un texte
// Les mots sont séparés par des espaces (split_whitespace)
// ------------------------------------------------------------
fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

// ------------------------------------------------------------
// Retourne le mot le plus long du texte
// Lifetime 'a : le résultat est une tranche du texte d'entrée
// ------------------------------------------------------------
fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte
        .split_whitespace()
        .max_by_key(|mot| mot.len())
        .unwrap_or("") // texte vide → chaîne vide
}

// ------------------------------------------------------------
// Vérifie si le texte est un palindrome
// Ignore la casse et les espaces
// "A man a plan a canal Panama" → vrai
// ------------------------------------------------------------
fn est_palindrome(texte: &str) -> bool {
    // Filtrer : garder uniquement les lettres, en minuscules
    let nettoye: Vec<char> = texte
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let n = nettoye.len();
    if n == 0 {
        return true;
    }

    // Comparer de l'extérieur vers le centre
    for i in 0..n / 2 {
        if nettoye[i] != nettoye[n - 1 - i] {
            return false;
        }
    }
    true
}

// ------------------------------------------------------------
// Retourne les N premiers mots sous forme de Vec<&str>
// Les éléments pointent vers des tranches du texte original
// ------------------------------------------------------------
fn premiers_mots<'a>(texte: &'a str, n: usize) -> Vec<&'a str> {
    texte.split_whitespace().take(n).collect()
}

// ------------------------------------------------------------
// Remplace toutes les occurrences de `de` par `vers`
// Retourne un nouveau String (owned) car la taille peut changer
// ------------------------------------------------------------
fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

// ------------------------------------------------------------
// Calcule les statistiques complètes d'un texte
// ------------------------------------------------------------
fn analyser(texte: &str) -> Statistiques {
    let nb_mots = compter_mots(texte);

    // Nombre de caractères hors espaces
    let nb_caracteres = texte.chars().filter(|c| !c.is_whitespace()).count();

    // Nombre de phrases : on compte '.', '!' et '?'
    let nb_phrases = texte.chars().filter(|c| matches!(c, '.' | '!' | '?')).count();

    // Mot le plus long (on clone car Statistiques doit posséder ce String)
    let mot_long = mot_le_plus_long(texte).to_string();

    // Fréquence des caractères (hors espaces, en minuscules)
    let mut freq: HashMap<char, usize> = HashMap::new();
    for c in texte.chars().filter(|c| !c.is_whitespace()) {
        let lower = c.to_ascii_lowercase();
        *freq.entry(lower).or_insert(0) += 1;
    }

    // Trier par fréquence décroissante, garder le top 5
    let mut freq_vec: Vec<(char, usize)> = freq.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    let mut frequence_chars = [(' ', 0usize); 5];
    for (i, &(c, n)) in freq_vec.iter().take(5).enumerate() {
        frequence_chars[i] = (c, n);
    }

    Statistiques {
        nb_mots,
        nb_caracteres,
        nb_phrases,
        mot_le_plus_long: mot_long,
        frequence_chars,
    }
}

// ------------------------------------------------------------
// Affichage des statistiques
// ------------------------------------------------------------
fn afficher_statistiques(stats: &Statistiques) {
    println!("  Mots            : {}", stats.nb_mots);
    println!("  Caractères      : {}", stats.nb_caracteres);
    println!("  Phrases         : {}", stats.nb_phrases);
    println!("  Mot le + long   : \"{}\"", stats.mot_le_plus_long);
    print!  ("  Top 5 chars     : ");
    for (c, n) in &stats.frequence_chars {
        if *n > 0 {
            print!("'{}':{}  ", c, n);
        }
    }
    println!();
}

// ============================================================
// Programme principal — démonstration de toutes les fonctions
// ============================================================
fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║   TP3 — Analyseur de Chaînes                ║");
    println!("╚══════════════════════════════════════════════╝\n");

    let texte1 = "Bonjour tout le monde. Comment allez-vous aujourd'hui? Très bien!";
    let texte2 = "Rust est un langage de programmation système moderne et performant.";
    let palindromes = ["kayak", "A man a plan a canal Panama", "Rust", "level", "bonjour"];

    // --- compter_mots ---
    println!("[ compter_mots ]");
    println!("  \"{}\"", texte1);
    println!("  → {} mots\n", compter_mots(texte1));

    // --- mot_le_plus_long ---
    println!("[ mot_le_plus_long ]");
    println!("  \"{}\"", texte2);
    println!("  → \"{}\"\n", mot_le_plus_long(texte2));

    // --- est_palindrome ---
    println!("[ est_palindrome ]");
    for p in &palindromes {
        println!("  \"{:<35}\" → {}", p, if est_palindrome(p) { "✅ palindrome" } else { "❌ non" });
    }
    println!();

    // --- premiers_mots ---
    println!("[ premiers_mots (n=4) ]");
    let premiers = premiers_mots(texte2, 4);
    println!("  {:?}\n", premiers);

    // --- remplacer ---
    println!("[ remplacer ]");
    let remplace = remplacer(texte2, "système", "systems");
    println!("  Avant  : {}", texte2);
    println!("  Après  : {}\n", remplace);

    // --- analyser ---
    println!("[ analyser ]");
    println!("  Texte : \"{}\"", texte1);
    let stats = analyser(texte1);
    afficher_statistiques(&stats);
}

// ============================================================
// Tests unitaires (identiques aux tests du sujet + extras)
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    // Tests fournis dans le sujet
    #[test]
    fn test_compter_mots() {
        assert_eq!(compter_mots("hello world"), 2);
        assert_eq!(compter_mots(""), 0);
        assert_eq!(compter_mots("   espaces   "), 1);
    }

    #[test]
    fn test_palindrome() {
        assert!(est_palindrome("kayak"));
        assert!(est_palindrome("A man a plan a canal Panama"));
        assert!(!est_palindrome("Rust"));
    }

    // Tests complémentaires
    #[test]
    fn test_palindrome_vide() {
        assert!(est_palindrome(""));
    }

    #[test]
    fn test_palindrome_un_char() {
        assert!(est_palindrome("a"));
    }

    #[test]
    fn test_mot_le_plus_long_basique() {
        assert_eq!(mot_le_plus_long("je suis programmeur"), "programmeur");
    }

    #[test]
    fn test_mot_le_plus_long_vide() {
        assert_eq!(mot_le_plus_long(""), "");
    }

    #[test]
    fn test_premiers_mots() {
        let mots = premiers_mots("un deux trois quatre cinq", 3);
        assert_eq!(mots, vec!["un", "deux", "trois"]);
    }

    #[test]
    fn test_premiers_mots_plus_que_disponible() {
        let mots = premiers_mots("un deux", 10);
        assert_eq!(mots.len(), 2);
    }

    #[test]
    fn test_remplacer() {
        assert_eq!(remplacer("hello world", "world", "Rust"), "hello Rust");
    }

    #[test]
    fn test_remplacer_absent() {
        assert_eq!(remplacer("bonjour", "xyz", "abc"), "bonjour");
    }

    #[test]
    fn test_analyser_nb_mots() {
        let stats = analyser("Bonjour monde.");
        assert_eq!(stats.nb_mots, 2);
    }

    #[test]
    fn test_analyser_nb_phrases() {
        let stats = analyser("Phrase un. Phrase deux! Question?");
        assert_eq!(stats.nb_phrases, 3);
    }

    #[test]
    fn test_analyser_mot_long() {
        let stats = analyser("je programme en Rust");
        assert_eq!(stats.mot_le_plus_long, "programme");
    }
}

