pub fn run() {
    let tache = creer_tache(String::from("Apprendre Rust"), 1);
    afficher_tache(tache); // ownership consumed

    let tache2 = creer_tache(String::from("Coder projet"), 2);
    let tache2 = marquer_complete(tache2);
    afficher_tache(tache2); // ownership consumed

    let tache3 = creer_tache(String::from("Lire documentation"), 3);
    let titre = extraire_titre(tache3);

    println!("Titre extrait: {}", titre);
}

// create task
fn creer_tache(titre: String, priorite: u8) -> (String, u8, bool) {
    (titre, priorite, false)
}

// display task
fn afficher_tache(tache: (String, u8, bool)) {
    let (titre, priorite, complete) = tache;

    println!(
        "Titre: {}, Priorite: {}, Complete: {}",
        titre, priorite, complete
    );
}

// mark task as completed
fn marquer_complete(tache: (String, u8, bool)) -> (String, u8, bool) {
    let (titre, priorite, _) = tache;

    (titre, priorite, true)
}

// extract title
fn extraire_titre(tache: (String, u8, bool)) -> String {
    let (titre, _, _) = tache;

    titre
}