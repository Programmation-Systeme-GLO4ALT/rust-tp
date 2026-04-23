mod analyse;

use analyse::*;

fn main() {
    let texte = "Hello world! Rust is fun. Level up your skills.";

    println!("Word count: {}", compter_mots(texte));
    println!("Longest word: {}", mot_le_plus_long(texte));
    println!("Is palindrome? {}", est_palindrome("level"));
    println!("First 3 words: {:?}", premiers_mots(texte, 3));
    println!(
        "Replace 'Rust' with 'Programming': {}",
        remplacer(texte, "Rust", "Programming")
    );

    let stats = analyser(texte);
    println!("Statistics:");
    println!("Words: {}", stats.nb_mots);
    println!("Characters: {}", stats.nb_caracteres);
    println!("Sentences: {}", stats.nb_phrases);
    println!("Longest word: {}", stats.mot_le_plus_long);
    println!("Top 5 chars: {:?}", stats.frequence_chars);
}