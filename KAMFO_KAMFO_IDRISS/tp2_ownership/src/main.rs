fn creer_tache(titre: String, priorite: u8) -> (String, u8, bool) {
    (titre, priorite, false)
}

fn marquer_complete(mut tache: (String, u8, bool)) -> (String, u8, bool) {
    tache.2 = true;
    tache // On retourne le tuple pour rendre l'ownership
}

fn extraire_titre(tache: (String, u8, bool)) -> String {
    tache.0 // Consomme le tuple, extrait uniquement le titre
}

fn afficher_tache(tache: (String, u8, bool)) {
    println!("Tâche : '{}' | Priorité : {} | Complète : {}", tache.0, tache.1, tache.2);
    // Le tuple est détruit (droppé) à la fin de cette fonction
}

fn main() {
    let tache1 = creer_tache(String::from("Apprendre Rust"), 1);
    let tache1_mise_a_jour = marquer_complete(tache1); // Transfert d'ownership
    afficher_tache(tache1_mise_a_jour); // Transfert final pour affichage
}