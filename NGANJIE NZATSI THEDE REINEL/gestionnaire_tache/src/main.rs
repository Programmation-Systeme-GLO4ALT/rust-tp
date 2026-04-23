
fn creer_tache(titre: String, priorite: u8) -> (String, u8, bool) {
    (titre, priorite, false)
}


fn afficher_tache(tache: (String, u8, bool)) {
    let (titre, priorite, complete) = tache;

    println!("Titre: {}", titre);
    println!("Priorité: {}", priorite);
    println!("Complète: {}", complete);
}


fn marquer_complete(tache: (String, u8, bool)) -> (String, u8, bool) {
    let (titre, priorite, _) = tache;

    (titre, priorite, true)
}


fn extraire_titre(tache: (String, u8, bool)) -> String {
    let (titre, _, _) = tache;

    titre
}

fn main() {


    let tache = creer_tache(String::from("Apprendre Rust"), 1);


    let tache = marquer_complete(tache);


    afficher_tache(tache);


    let tache2 = creer_tache(String::from("Faire les exercices"), 2);


    let titre = extraire_titre(tache2);

    println!("Titre extrait: {}", titre);
}