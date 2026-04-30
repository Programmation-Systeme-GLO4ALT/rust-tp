use std::collections::HashMap;

#[derive(Debug)]
struct Statistiques {
    nb_mots: usize,
    nb_caracteres: usize,
    nb_phrases: usize,
    mot_le_plus_long: String,
    frequence_chars: [(char, usize); 5],
}

fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    let mut plus_long = "";

    for mot in texte.split_whitespace() {
        if mot.chars().count() > plus_long.chars().count() {
            plus_long = mot;
        }
    }

    plus_long
}

fn est_palindrome(texte: &str) -> bool {
    let normalise: String = texte
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();

    normalise.chars().eq(normalise.chars().rev())
}

fn premiers_mots(texte: &str, n: usize) -> Vec<&str> {
    texte.split_whitespace().take(n).collect()
}

fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

fn analyser(texte: &str) -> Statistiques {
    let nb_mots = compter_mots(texte);
    let nb_caracteres = texte.chars().count();
    let nb_phrases = texte
        .chars()
        .filter(|c| matches!(c, '.' | '!' | '?'))
        .count();
    let mot_long = mot_le_plus_long(texte).to_string();

    let mut freqs: HashMap<char, usize> = HashMap::new();
    for c in texte.chars() {
        if !c.is_whitespace() {
            for c_min in c.to_lowercase() {
                *freqs.entry(c_min).or_insert(0) += 1;
            }
        }
    }

    let mut frequences: Vec<(char, usize)> = freqs.into_iter().collect();
    frequences.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let mut top5 = [(' ', 0); 5];
    for (index, (c, count)) in frequences.into_iter().take(5).enumerate() {
        top5[index] = (c, count);
    }

    Statistiques {
        nb_mots,
        nb_caracteres,
        nb_phrases,
        mot_le_plus_long: mot_long,
        frequence_chars: top5,
    }
}

fn afficher_analyse(texte: &str) {
    println!("Texte analyse :");
    println!("{texte}");
    println!();

    let stats = analyser(texte);

    println!("Nombre de mots        : {}", stats.nb_mots);
    println!("Nombre de caracteres  : {}", stats.nb_caracteres);
    println!("Nombre de phrases     : {}", stats.nb_phrases);
    println!("Mot le plus long      : {}", stats.mot_le_plus_long);
    println!("Top 5 caracteres      : {:?}", stats.frequence_chars);

    println!("Premiers 4 mots       : {:?}", premiers_mots(texte, 4));
    println!(
        "Remplacement Rust->RUST : {}",
        remplacer(texte, "Rust", "RUST")
    );
    println!(
        "Est palindrome ?      : {}",
        est_palindrome(texte)
    );
}

fn main() {
    let texte = "Rust est rapide, sur et moderne. Rust evite de nombreux bugs memoire! Rust simplifie aussi l'analyse.";
    afficher_analyse(texte);

    println!();
    let palindrome = "A man a plan a canal Panama";
    println!(
        "\"{}\" est un palindrome ? {}",
        palindrome,
        est_palindrome(palindrome)
    );
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
        assert_eq!(mot_le_plus_long("un mot gigantesque"), "gigantesque");
        assert_eq!(mot_le_plus_long(""), "");
    }

    #[test]
    fn test_premiers_mots() {
        assert_eq!(premiers_mots("un deux trois quatre", 2), vec!["un", "deux"]);
    }

    #[test]
    fn test_remplacer() {
        assert_eq!(remplacer("bonjour rust", "rust", "Rust"), "bonjour Rust");
    }

    #[test]
    fn test_analyser() {
        let stats = analyser("Rust est fiable. Rust est rapide!");
        assert_eq!(stats.nb_mots, 6);
        assert_eq!(stats.nb_phrases, 2);
        assert_eq!(stats.mot_le_plus_long, "fiable.");
    }
}