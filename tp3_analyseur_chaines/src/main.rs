// TP 3 — Analyseur de chaînes : références, slices, lifetimes

struct Statistiques {
    nb_mots: usize,
    nb_caracteres: usize,
    nb_phrases: usize,
    mot_le_plus_long: String,
    frequence_chars: [(char, usize); 5], // top 5 caractères (hors espaces)
}

fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte
        .split_whitespace()
        .max_by_key(|m| m.len())
        .unwrap_or("")
}

fn est_palindrome(texte: &str) -> bool {
    // Normalise : minuscules, lettres/chiffres seulement
    let normalise: String = texte
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    let inverse: String = normalise.chars().rev().collect();
    normalise == inverse
}

fn premiers_mots<'a>(texte: &'a str, n: usize) -> Vec<&'a str> {
    texte.split_whitespace().take(n).collect()
}

fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

fn analyser(texte: &str) -> Statistiques {
    let nb_mots = compter_mots(texte);
    let nb_caracteres = texte.chars().count();
    // Phrases : terminées par . ! ?
    let nb_phrases = texte.chars().filter(|&c| c == '.' || c == '!' || c == '?').count();
    let mot_le_plus_long = mot_le_plus_long(texte).to_string();

    // Fréquence des caractères (hors espaces)
    let mut freq: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
    for c in texte.chars().filter(|c| !c.is_whitespace()) {
        *freq.entry(c).or_insert(0) += 1;
    }
    let mut freq_vec: Vec<(char, usize)> = freq.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let mut frequence_chars = [(' ', 0usize); 5];
    for (i, &(c, n)) in freq_vec.iter().take(5).enumerate() {
        frequence_chars[i] = (c, n);
    }

    Statistiques {
        nb_mots,
        nb_caracteres,
        nb_phrases,
        mot_le_plus_long,
        frequence_chars,
    }
}

fn afficher_stats(stats: &Statistiques) {
    println!("--- Statistiques ---");
    println!("Mots        : {}", stats.nb_mots);
    println!("Caractères  : {}", stats.nb_caracteres);
    println!("Phrases     : {}", stats.nb_phrases);
    println!("Mot le plus long : '{}'", stats.mot_le_plus_long);
    println!("Top 5 caractères :");
    for (c, n) in &stats.frequence_chars {
        if *n > 0 {
            println!("  '{}' : {}", c, n);
        }
    }
}

fn main() {
    let texte = "Rust est un langage de programmation système. \
                 Il garantit la sécurité mémoire sans garbage collector. \
                 Rust est rapide, fiable et productif!";

    println!("Texte analysé :\n\"{}\"\n", texte);

    println!("Nombre de mots : {}", compter_mots(texte));
    println!("Mot le plus long : '{}'", mot_le_plus_long(texte));
    println!("Est palindrome : {}", est_palindrome(texte));
    println!("Palindrome 'kayak' : {}", est_palindrome("kayak"));
    println!(
        "Palindrome 'A man a plan a canal Panama' : {}",
        est_palindrome("A man a plan a canal Panama")
    );

    let premiers = premiers_mots(texte, 4);
    println!("4 premiers mots : {:?}", premiers);

    let remplace = remplacer(texte, "Rust", "🦀 Rust");
    println!("Après remplacement : \"{}\"", &remplace[..60]);

    println!();
    let stats = analyser(texte);
    afficher_stats(&stats);
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
        assert_eq!(mot_le_plus_long("Rust est génial"), "génial");
        assert_eq!(mot_le_plus_long(""), "");
    }

    #[test]
    fn test_premiers_mots() {
        let mots = premiers_mots("un deux trois quatre", 2);
        assert_eq!(mots, vec!["un", "deux"]);
    }

    #[test]
    fn test_remplacer() {
        assert_eq!(remplacer("hello world", "world", "Rust"), "hello Rust");
    }
}
