// TP 2 — Gestion Mémoire : ownership, move, Copy
// Partie A : démonstration des erreurs corrigées
// Partie B : gestionnaire de tâches sans références

// --- Partie A : corrections des programmes d'ownership ---

fn demo_ownership() {
    println!("=== Partie A : Ownership ===\n");

    // Programme 1 corrigé : on transfère ownership à v2, v n'est plus valide.
    // Solution : utiliser v2 ou cloner v avant le move.
    let v = vec![1, 2, 3];
    let v2 = v; // move : v invalide
    println!("Programme 1 — v2 (après move) : {:?}", v2);
    // println!("{}", v.len()); // ERREUR : borrow of moved value

    // Programme 2 corrigé sans clone() : on retourne le Vec depuis la fonction
    // pour récupérer l'ownership après l'appel.
    fn somme_et_retour(v: Vec<i32>) -> (i32, Vec<i32>) {
        let s = v.iter().sum();
        (s, v) // on rend le Vec avec la somme
    }

    let nombres = vec![1, 2, 3, 4, 5];
    let (s, nombres) = somme_et_retour(nombres);
    println!("Programme 2 — Somme : {}, Vecteur : {:?}\n", s, nombres);
}

// --- Partie B : gestionnaire de tâches ---
// Tâche représentée par un tuple (titre: String, priorité: u8, complète: bool)

fn creer_tache(titre: String, priorite: u8) -> (String, u8, bool) {
    (titre, priorite, false)
}

// Consomme la tâche pour l'afficher (ownership transféré)
fn afficher_tache(tache: &(String, u8, bool)) {
    let statut = if tache.2 { "✓" } else { "○" };
    println!("[{}] {} (priorité: {})", statut, tache.0, tache.1);
}

// Consomme et retourne une nouvelle tâche marquée complète
fn marquer_complete(tache: (String, u8, bool)) -> (String, u8, bool) {
    (tache.0, tache.1, true)
}

// Consomme la tâche et retourne le titre
fn extraire_titre(tache: (String, u8, bool)) -> String {
    tache.0
}

fn demo_taches() {
    println!("=== Partie B : Gestionnaire de tâches ===\n");

    let t1 = creer_tache(String::from("Apprendre Rust"), 1);
    let t2 = creer_tache(String::from("Lire le Rust Book"), 2);
    let t3 = creer_tache(String::from("Faire les TPs"), 1);

    println!("Tâches créées :");
    afficher_tache(&t1);
    afficher_tache(&t2);
    afficher_tache(&t3);

    // Marquer t1 comme complète (move + retour)
    let t1 = marquer_complete(t1);
    println!("\nAprès complétion de t1 :");
    afficher_tache(&t1);
    afficher_tache(&t2);

    // Extraire le titre de t3 (consomme t3)
    let titre = extraire_titre(t3);
    println!("\nTitre extrait : '{}'", titre);
    // t3 n'est plus accessible ici

    // Démonstration Copy vs Move
    println!("\n--- Copy vs Move ---");
    let x: i32 = 42;
    let y = x; // Copy : x toujours valide
    println!("i32 Copy — x={}, y={}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Clone explicite
    println!("String Clone — s1={}, s2={}", s1, s2);
}

fn main() {
    demo_ownership();
    demo_taches();
}
