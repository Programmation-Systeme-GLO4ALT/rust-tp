fn compter_mots(texte: &str) -> usize {
    texte.split_whitespace().count()
}

fn mot_le_plus_long<'a>(texte: &'a str) -> &'a str {
    texte.split_whitespace().max_by_key(|mot| mot.len()).unwrap_or("")
}

fn est_palindrome(texte: &str) -> bool {
    let t: String = texte.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect();
    let rev: String = t.chars().rev().collect();
    t == rev && !t.is_empty()
}

fn premiers_mots(texte: &str, n: usize) -> Vec<&str> {
    texte.split_whitespace().take(n).collect()
}

fn remplacer(texte: &str, de: &str, vers: &str) -> String {
    texte.replace(de, vers)
}

fn main() {
    let texte = "Rust est un langage fantastique. Rust est rapide.";
    println!("Nombre de mots : {}", compter_mots(texte));
    println!("Mot le plus long : {}", mot_le_plus_long(texte));
    println!("Est palindrome 'kayak' ? {}", est_palindrome("kayak"));
    println!("Premiers mots : {:?}", premiers_mots(texte, 3));
    println!("Remplacement : {}", remplacer(texte, "Rust", "C++"));
}