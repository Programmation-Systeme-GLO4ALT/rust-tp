// Compte les mots dans un texte
fn compter_mots(texte: &str) -> usize {
    if(texte.is_empty()){
        return 0;
    } else {
        return texte.split_whitespace().count();
    }
}

// Retourne le mot le plus long
fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str { 
    texte.split_whitespace().max_by_key(|mot| mot.len()).unwrap_or("")
}

// Vérifie si le texte est un palindrome
fn est_palindrome(texte: &str) -> bool { 
    // Convertir en minuscules et garder seulement les caractères alphanumériques
    let nettoye: String = texte.chars().filter(|c| c.is_alphanumeric()).flat_map(|c| c.to_lowercase()).collect();

    // Vérifier si c'est un palindrome
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
fn analyser(texte: &str) -> Statistiques { 
    let nb_mots = compter_mots(texte);
    let nb_caracteres = texte.chars().count();
    let nb_phrases = texte.matches('.').count() + texte.matches('!').count() + texte.matches('?').count();
    let mot_le_plus_long = mot_le_plus_long(texte).to_string();

    // Calcul de la fréquence des caractères
    let mut frequence = std::collections::HashMap::new();
    for c in texte.chars().filter(|c| !c.is_whitespace()) {
        *frequence.entry(c).or_insert(0) += 1;
    }

    // Trier par fréquence et prendre les 5 premiers
    let mut frequence_vec: Vec<(char, usize)> = frequence.into_iter().collect();
    frequence_vec.sort_by(|a, b| b.1.cmp(&a.1)); // trier par fréquence décroissante
    let frequence_chars: [(char, usize); 5] = [
        frequence_vec.get(0).cloned().unwrap_or((' ', 0)),
        frequence_vec.get(1).cloned().unwrap_or((' ', 0)),
        frequence_vec.get(2).cloned().unwrap_or((' ', 0)),
        frequence_vec.get(3).cloned().unwrap_or((' ', 0)),
        frequence_vec.get(4).cloned().unwrap_or((' ', 0)),
    ];

    Statistiques {
        nb_mots,
        nb_caracteres,
        nb_phrases,
        mot_le_plus_long,
        frequence_chars,
    }
}

struct Statistiques {
    nb_mots: usize,
    nb_caracteres: usize,
    nb_phrases: usize,
    mot_le_plus_long: String,
    frequence_chars: [(char, usize); 5],  // top 5
}

// pour executer les tests : cargo test
// pour executer un test spécifique : cargo test test_compter_mots
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
}