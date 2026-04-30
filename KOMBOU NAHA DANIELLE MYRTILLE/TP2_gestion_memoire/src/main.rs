// Ce TP2 contient systematiquement 2 partie a savoir A et B

//Partie A

//programme avec erreur
// fn main() {
//     let v = vec![1, 2, 3];
//     let v2 = v;
//     println!("Longueur : {}", v.len());
// }


//programme corriger
// fn main() {
//     let v = vec![1, 2, 3];
//     let v2 = v;                        // v est moved vers v2
//     println!("Longueur : {}", v2.len()); // on utilise v2 maintenant
// }


//programme avec erreur
// fn somme(v: Vec<i32>) -> i32 {
//     v.iter().sum()
// }

// fn main() {
//     let nombres = vec![1, 2, 3, 4, 5];
//     let s = somme(nombres);
//     println!("Somme : {}, Vecteur : {:?}", s, nombres);
// }


//programme corriger
// fn somme(v: &Vec<i32>) -> i32 {   // & = on emprunte, on ne prend pas ownership
//     v.iter().sum()
// }

// fn main() {
//     let nombres = vec![1, 2, 3, 4, 5];
//     let s = somme(&nombres);       // & = on passe une référence
//     println!("Somme : {}, Vecteur : {:?}", s, nombres); // nombres toujours valide ✅
// }


//partie B

// Créer une nouvelle tâche
fn creer_tache(titre: String, priorite: u8) -> (String, u8, bool) {
    (titre, priorite, false)   
}

// Afficher une tâche 
fn afficher_tache(tache: (String, u8, bool)) {
    println!(
        "[{}] {} (priorité: {})",
        if tache.2 { "✓" } else { " " }, 
        tache.0,                            
        tache.1                             
    );
}

// Marquer une tâche comme complète 
fn marquer_complete(tache: (String, u8, bool)) -> (String, u8, bool) {
    (tache.0, tache.1, true)   
}

// Extraire uniquement le titre 
fn extraire_titre(tache: (String, u8, bool)) -> String {
    tache.0   // on retourne le titre, le reste est droppé
}

fn main() {
    // 1. Créer des tâches
    let tache1 = creer_tache(String::from("Apprendre Rust"), 1);
    let tache2 = creer_tache(String::from("Faire les courses"), 3);
    let tache3 = creer_tache(String::from("Lire un livre"), 2);

    // 2. Marquer tache1 comme complète

    let tache1 = marquer_complete(tache1);

    // 3. Afficher toutes les tâches
    
    afficher_tache(tache1);
    // println!("{}", tache1.0);   erreur 1 pour tester
    afficher_tache(tache2);

    // 4. Extraire le titre de tache3
    let titre = extraire_titre(tache3);
    // afficher_tache(tache3);         erreur 2 pour les test
    println!("Titre extrait : {}", titre);
}