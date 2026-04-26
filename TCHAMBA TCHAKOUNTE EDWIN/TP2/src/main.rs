// =============================================================
// TP2 — Gestion Mémoire Manuelle (sans références)
// Tchamba Tchakounte Edwin
// =============================================================

// =============================================================
// PARTIE A — Débogage d'erreurs d'ownership
// =============================================================

// Programme 1 corrigé : on utilise v2 (le nouveau propriétaire)
// Le Vec a été MOVED de v vers v2 lors de `let v2 = v;`
fn programme_1_corrige() {
    let v = vec![1, 2, 3];
    let v2 = v;                            // MOVE : v n'existe plus
    println!("[Prog 1] Longueur : {}", v2.len());
}

// Programme 2 corrigé : la fonction RETOURNE le Vec
// Ainsi l'ownership revient au caller après l'opération
fn somme(v: Vec<i32>) -> (i32, Vec<i32>) {
    let total: i32 = v.iter().sum();
    (total, v)                              // on rend l'ownership
}

fn programme_2_corrige() {
    let nombres = vec![1, 2, 3, 4, 5];
    let (s, nombres) = somme(nombres);     // on récupère le vec
    println!("[Prog 2] Somme : {}, Vecteur : {:?}", s, nombres);
}

// =============================================================
// PARTIE B — Gestionnaire de tâches (sans références)
// Représentation d'une tâche : (titre: String, priorité: u8, complétée: bool)
// =============================================================

// Crée une nouvelle tâche, non complétée par défaut
fn creer_tache(titre: String, priorite: u8) -> (String, u8, bool) {
    (titre, priorite, false)
}

// Affiche une tâche. Consomme l'ownership (la tâche est détruite après).
fn afficher_tache(tache: (String, u8, bool)) {
    let (titre, priorite, complete) = tache;
    let statut = if complete { "[x]" } else { "[ ]" };
    println!("{} prio={} : {}", statut, priorite, titre);
}

// Marque une tâche comme complète. Prend l'ownership et le rend modifié.
fn marquer_complete(tache: (String, u8, bool)) -> (String, u8, bool) {
    let (titre, priorite, _) = tache;
    (titre, priorite, true)
}

// Extrait le titre d'une tâche. Consomme la tâche, ne renvoie que le titre.
fn extraire_titre(tache: (String, u8, bool)) -> String {
    let (titre, _, _) = tache;
    titre
}

fn main() {
    println!("=== PARTIE A — Débogage ===");
    programme_1_corrige();
    programme_2_corrige();

    println!("\n=== PARTIE B — Gestionnaire de tâches ===");

    // Création de plusieurs tâches
    let t1 = creer_tache(String::from("Acheter du pain"), 3);
    let t2 = creer_tache(String::from("Faire le TP Rust"), 1);
    let t3 = creer_tache(String::from("Appeler maman"), 2);

    // On clone t1 avant de l'afficher car afficher_tache consomme.
    // Sinon t1 serait inutilisable après.
    afficher_tache(t1.clone());
    afficher_tache(t2.clone());
    afficher_tache(t3.clone());

    // On marque t2 comme complète : transfert puis récupération
    println!("\n>>> On marque t2 comme complète");
    let t2 = marquer_complete(t2);
    afficher_tache(t2);

    // On extrait le titre de t3 : consomme t3
    println!("\n>>> Extraction du titre de t3");
    let titre_t3 = extraire_titre(t3);
    println!("Titre extrait : \"{}\"", titre_t3);
    // t3 n'existe plus ici — la tentative de l'utiliser causerait une erreur

    // t1 est encore valide grâce au clone() au-dessus
    println!("\n>>> t1 est toujours valide :");
    afficher_tache(t1);
}
