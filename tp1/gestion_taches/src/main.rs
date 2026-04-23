// Créer une tâche
fn creer_tache(titre: String, priorite: u8) -> (String, u8, bool) {
    (titre, priorite, false)
}

// Afficher une tâche (prend ownership)
fn afficher_tache(tache: (String, u8, bool)) {
    println!(
        "Titre: {}, Priorité: {}, Terminée: {}",
        tache.0, tache.1, tache.2
    );
}

// Marquer comme complète
fn marquer_complete(tache: (String, u8, bool)) -> (String, u8, bool) {
    (tache.0, tache.1, true)
}

// Extraire le titre (perd la tâche)
fn extraire_titre(tache: (String, u8, bool)) -> String {
    tache.0
}

// Programme principal
fn main() {
    println!("Bienvenue dans mon gestionnaire de tâches  !");
    let tache1 = creer_tache(String::from("Apprendre Rust"), 1);

    // afficher (consomme)
    afficher_tache(tache1);

    // recréer car ownership perdu
    let tache2 = creer_tache(String::from("Faire TP"), 2);

    let tache2 = marquer_complete(tache2);

    afficher_tache(tache2);

    let tache3 = creer_tache(String::from("Lire livre"), 3);

    let titre = extraire_titre(tache3);

    println!("Titre extrait : {}", titre);
}