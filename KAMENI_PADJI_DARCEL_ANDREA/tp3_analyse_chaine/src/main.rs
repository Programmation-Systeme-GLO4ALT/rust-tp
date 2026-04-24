// TP3 — Exemple d'utilisation
use tp3_analyseur_chaine::*;

fn main() {
    let texte = "Hello world! Rust is awesome.";
    println!("Nombre de mots : {}", compter_mots(texte));
    println!("Mot le plus long : {}", mot_le_plus_long(texte));
    println!("Est-ce un palindrome ? {}", est_palindrome("kayak"));
    println!("2 premiers mots : {:?}", premiers_mots(texte, 2));
    println!("Remplacement : {}", remplacer(texte, "world", "Rust"));
    let stats = analyser(texte);
    println!("Statistiques : {:#?}", stats);
}