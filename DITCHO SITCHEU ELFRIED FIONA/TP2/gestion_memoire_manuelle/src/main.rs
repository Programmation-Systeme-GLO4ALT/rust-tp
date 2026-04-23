// =================================================================
// TP 2 — Gestion Mémoire Manuelle  (Séance 2)
// =================================================================
// Thèmes : ownership, move semantics, types Copy vs Move
// cargo new tp2_memoire && cp tp2_memoire.rs tp2_memoire/src/main.rs
// cd tp2_memoire && cargo test
// =================================================================

// -----------------------------------------------------------------
//  PARTIE A — Corrections des programmes bugués
// -----------------------------------------------------------------

// Programme 1 — ERREUR E0382 : borrow of moved value
// Cause : 'v' est moved dans 'v2', puis v.len() est appelé sur une
//         variable invalide.
// Correction : cloner v AVANT de le déplacer.
fn programme1_corrige() {
    let v = vec![1, 2, 3];
    let _v2 = v.clone(); // copie profonde : v reste propriétaire
    println!("Longueur : {}", v.len()); // ✓ v toujours valide
}

// Programme 2 — ERREUR E0382 : borrow of moved value
// Cause : 'nombres' est moved dans somme(), puis utilisé dans println!.
// Contrainte : PAS de références (&).
// Correction : somme() rend l'ownership du vecteur en même temps que
//              le résultat (retour par tuple).
fn somme_et_retour(v: Vec<i32>) -> (i32, Vec<i32>) {
    let s: i32 = v.iter().sum();
    (s, v) // on transfère l'ownership en retour
}

fn programme2_corrige() {
    let nombres = vec![1, 2, 3, 4, 5];
    let (s, nombres) = somme_et_retour(nombres); // on récupère les deux
    println!("Somme : {}, Vecteur : {:?}", s, nombres); // ✓
}

// -----------------------------------------------------------------
//  PARTIE B — Gestionnaire de tâches SANS références (&)
// -----------------------------------------------------------------

// Représentation d'une tâche : (titre, priorité, complétée)
type Tache = (String, u8, bool);

/// Crée une nouvelle tâche non complète.
fn creer_tache(titre: String, priorite: u8) -> Tache {
    (titre, priorite, false)
}

/// Affiche la tâche, puis retourne l'ownership pour pouvoir la réutiliser.
fn afficher_tache(tache: Tache) -> Tache {
    let statut = if tache.2 { "✓" } else { "○" };
    println!("[{}] {} (priorité {})", statut, tache.0, tache.1);
    tache
}

/// Retourne une copie de la tâche marquée comme complète.
fn marquer_complete(tache: Tache) -> Tache {
    (tache.0, tache.1, true)
}

/// Consomme la tâche et retourne uniquement son titre.
fn extraire_titre(tache: Tache) -> String {
    tache.0 // move le String, les autres champs sont droppés
}

fn main() {
    println!("=== TP 2 — Gestion Mémoire Manuelle ===\n");

    // --- Partie A ---
    println!("-- Partie A : corrections d'erreurs d'ownership --");
    programme1_corrige();
    programme2_corrige();

    // --- Partie B ---
    println!("\n-- Partie B : gestionnaire de tâches (sans &) --");

    let t1 = creer_tache(String::from("Apprendre Rust"), 1);
    let t2 = creer_tache(String::from("Lire le Rust Book"), 2);
    let t3 = creer_tache(String::from("Faire les TPs"), 1);

    println!("\nTâches initiales :");
    let t1 = afficher_tache(t1);
    let t2 = afficher_tache(t2);
    let t3 = afficher_tache(t3);

    // Marquer t1 comme complète
    let t1 = marquer_complete(t1);

    println!("\nAprès complétion de t1 :");
    let t1 = afficher_tache(t1);
    let _  = afficher_tache(t2); // consommée ici
    let _  = afficher_tache(t3); // consommée ici

    // Extraire le titre de t1 (la consomme)
    let titre = extraire_titre(t1);
    println!("\nTitre extrait : '{}'", titre);
    // t1 n'existe plus ici — son ownership a été transféré
}

#[cfg(test)]
mod tests_tp2 {
    use super::*;

    #[test]
    fn test_creer_tache() {
        let t = creer_tache(String::from("Test"), 3);
        assert_eq!(t.0, "Test");
        assert_eq!(t.1, 3);
        assert!(!t.2);
    }

    #[test]
    fn test_marquer_complete() {
        let t = creer_tache(String::from("Test"), 1);
        let t = marquer_complete(t);
        assert!(t.2);
        assert_eq!(t.0, "Test"); // titre préservé
    }

    #[test]
    fn test_extraire_titre() {
        let t = creer_tache(String::from("Mon titre"), 2);
        let titre = extraire_titre(t);
        assert_eq!(titre, "Mon titre");
        // 't' est invalide ici (moved) — le compilateur le garantit
    }

    #[test]
    fn test_somme_sans_clone() {
        let v = vec![1, 2, 3, 4, 5];
        let (s, v) = somme_et_retour(v);
        assert_eq!(s, 15);
        assert_eq!(v.len(), 5); // vecteur toujours accessible
    }
}
