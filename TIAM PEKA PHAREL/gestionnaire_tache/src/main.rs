fn creer_tache(titre: String, priorite: u8) -> (String, u8, bool) { 
    return (titre, priorite, false)
}

fn afficher_tache(tache: (String, u8, bool)) { 
    let (titre, priorite, complete) = tache;
    println!("Tâche: {}, Priorité: {}, Complète: {}", titre, priorite, complete);
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
    println!("Gestionnaire de Tâches");

    let tache0 = creer_tache(String::from("aller en balade"), 1);
    afficher_tache(tache0);

    let tache1 = creer_tache(String::from("Faire une course"), 2);
    let titre_tache1 = extraire_titre(tache1);
    println!("Titre de la tâche 1: {}", titre_tache1);

    let tache2 = creer_tache(String::from("regarder un film"), 3);
    let tache2_complete = marquer_complete(tache2);
    afficher_tache(tache2_complete);

    
}
