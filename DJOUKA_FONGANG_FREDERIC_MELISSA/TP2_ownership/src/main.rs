fn creer_tache(titre: String, priorite: u8) -> (String, u8, bool) {
    (titre, priorite, false)
}

fn afficher_tache(tache: (String, u8, bool)) {
    println!("Titre: {}, Priorite: {}, Complete: {}", tache.0, tache.1, tache.2);
}

fn marquer_complete(tache: (String, u8, bool)) -> (String, u8, bool) {
    (tache.0, tache.1, true)
}

fn extraire_titre(tache: (String, u8, bool)) -> String {
    tache.0
}

fn main() {
    let t1 = creer_tache(String::from("Apprendre Rust"), 1);
    let t1 = marquer_complete(t1);

    afficher_tache(t1);

    let t2 = creer_tache(String::from("Coder projet"), 2);
    let titre = extraire_titre(t2);

    println!("Titre de l' extrait: {}", titre);
}
