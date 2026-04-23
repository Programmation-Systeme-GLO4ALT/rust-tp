fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte.split_whitespace().max_by_key(|mot| mot.len()).unwrap_or("")
}

fn est_palindrome(texte: &str) -> bool {
    let clean: String = texte
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    clean == clean.chars().rev().collect::<String>()
}

fn premiers_mots(texte: &str, n: usize) -> Vec<&str> {
    texte.split_whitespace().take(n).collect()
}

fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

fn main() {
    let texte = "Rust est rapide et sécurise";

    println!("Mots: {}", compter_mots(texte));
    println!("Plus long: {}", mot_le_plus_long(texte));
    println!("Palindrome: {}", est_palindrome("kayak"));
    println!("Premiers mots: {:?}", premiers_mots(texte, 2));
    println!("Remplacement: {}", remplacer(texte, "Rust", "Go"));
}
