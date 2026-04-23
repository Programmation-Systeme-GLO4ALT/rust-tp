// ============================================================
// TP 2 — Gestion Mémoire & Ownership
// Programmation Système avec Rust — GL4 ENSPD 2025-2026
// ============================================================
//
// Partie A : Débogage d'erreurs d'ownership
//   - Programme 1 : move sémantique sur Vec
//   - Programme 2 : passage par référence au lieu de move
//
// Partie B : Gestionnaire de tâches sans références (&)
//   Une tâche est un tuple (String, u8, bool) :
//     - String : titre
//     - u8     : priorité (1 = basse, 5 = haute)
//     - bool   : complète ?
// ============================================================

// ============================================================
// PARTIE A — Analyse et correction des erreurs d'ownership
// ============================================================

// ------------------------------------------------------------
// Programme 1 ORIGINAL (ne compile pas) :
//
//   let v = vec![1, 2, 3];
//   let v2 = v;               // v est MOVED dans v2
//   println!("{}", v.len());  // ERREUR : v n'est plus valide
//
// EXPLICATION : En Rust, l'affectation d'un Vec transfère la
// propriété (move). v n'est plus valide après "let v2 = v".
// Le compilateur garantit qu'il n'y a qu'un seul propriétaire.
//
// CORRECTION : utiliser clone() ou ne plus utiliser v après le move
// ------------------------------------------------------------
#[allow(dead_code)]
fn programme1_corrige() {
    let v = vec![1, 2, 3];
    let v2 = v.clone(); // Clone explicite : v2 obtient sa propre copie
    println!("v.len()  = {}", v.len()); // v est encore valide
    println!("v2.len() = {}", v2.len());
}

// ------------------------------------------------------------
// Programme 2 ORIGINAL (ne compile pas) :
//
//   fn somme(v: Vec<i32>) -> i32 { v.iter().sum() }
//   let nombres = vec![1,2,3,4,5];
//   let s = somme(nombres);           // nombres est MOVED
//   println!("{:?}", nombres);        // ERREUR : nombres moved
//
// EXPLICATION : somme() prend le Vec par valeur → move.
// Après l'appel, nombres n'existe plus dans main.
//
// CORRECTION SANS clone() : passer une référence &[i32] au lieu
// du Vec<i32>. La fonction emprunte les données sans les posséder.
// ------------------------------------------------------------
fn somme(v: &[i32]) -> i32 {
    // On prend un slice en référence → pas de move
    v.iter().sum()
}

#[allow(dead_code)]
fn programme2_corrige() {
    let nombres = vec![1, 2, 3, 4, 5];
    let s = somme(&nombres); // On passe une référence : nombres reste valide
    println!("Somme : {}, Vecteur : {:?}", s, nombres); // OK !
}

// ============================================================
// PARTIE B — Gestionnaire de tâches
// Type d'une tâche : (String, u8, bool)
//   .0 = titre
//   .1 = priorité (1..=5)
//   .2 = complète
// RÈGLE : pas de & dans cet exercice → ownership explicite
// ============================================================

// Crée une nouvelle tâche (incomplète par défaut)
fn creer_tache(titre: String, priorite: u8) -> (String, u8, bool) {
    assert!(priorite >= 1 && priorite <= 5, "Priorité doit être entre 1 et 5");
    (titre, priorite, false)
}

// Affiche une tâche — consomme la tâche (move)
// Pour afficher sans consommer, on clonerait avant l'appel
fn afficher_tache(tache: (String, u8, bool)) {
    let statut = if tache.2 { "✅ Complète" } else { "⏳ En cours" };
    let priorite_label = match tache.1 {
        1 => "Très basse",
        2 => "Basse",
        3 => "Normale",
        4 => "Haute",
        5 => "Très haute",
        _ => "Inconnue",
    };
    println!(
        "  [{statut}] {} | Priorité : {} ({}/5)",
        tache.0, priorite_label, tache.1
    );
}

// Marque une tâche comme complète — prend ownership, retourne une nouvelle tâche
fn marquer_complete(tache: (String, u8, bool)) -> (String, u8, bool) {
    (tache.0, tache.1, true) // Reconstruit le tuple avec bool = true
}

// Extrait le titre — consomme la tâche et retourne son titre
fn extraire_titre(tache: (String, u8, bool)) -> String {
    tache.0 // Transfère ownership du String
}

// Retourne vrai si la tâche est de priorité haute ou très haute
fn est_haute_priorite(tache: &(String, u8, bool)) -> bool {
    // EXCEPTION PÉDAGOGIQUE : on prend une ref pour éviter de consommer
    // (dans un vrai contexte sans &, on clonerait puis consommerait)
    tache.1 >= 4
}

// ============================================================
// Programme principal : démonstration du gestionnaire
// ============================================================
fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║   TP2 — Gestionnaire de Tâches (Ownership)  ║");
    println!("╚══════════════════════════════════════════════╝\n");

    // --- Partie A : démonstration des corrections ---
    println!("[ Partie A — Corrections Ownership ]\n");

    // Programme 1 corrigé
    let v = vec![1, 2, 3];
    let v2 = v.clone();
    println!("Programme 1 : v={:?}, v2={:?}", v, v2);

    // Programme 2 corrigé
    let nombres = vec![1, 2, 3, 4, 5];
    let s = somme(&nombres);
    println!("Programme 2 : Somme={}, Vecteur={:?}\n", s, nombres);

    // --- Partie B : gestionnaire de tâches ---
    println!("[ Partie B — Tâches ]\n");

    // Création
    let t1 = creer_tache(String::from("Implémenter le lexer LGLo"), 5);
    let t2 = creer_tache(String::from("Rédiger le rapport TLC"), 4);
    let t3 = creer_tache(String::from("Réviser le cours Rust"), 3);
    let t4 = creer_tache(String::from("Faire le TP1 calculatrice"), 2);

    // Affichage (consomme les tâches → on clone celles qu'on veut garder)
    println!("Tâches créées :");
    afficher_tache(t1.clone());
    afficher_tache(t2.clone());
    afficher_tache(t3.clone());
    afficher_tache(t4.clone());

    // Marquer comme complète
    let t1_done = marquer_complete(t1); // t1 est moved ici
    let t4_done = marquer_complete(t4);

    println!("\nAprès completion :");
    afficher_tache(t1_done.clone());
    afficher_tache(t2.clone());
    afficher_tache(t3.clone());
    afficher_tache(t4_done.clone());

    // Haute priorité
    println!("\nTâches haute priorité :");
    for t in [&t1_done, &t2, &t3, &t4_done] {
        if est_haute_priorite(t) {
            println!("  → {}", t.0);
        }
    }

    // Extraire le titre (consomme la tâche)
    let titre = extraire_titre(t2);
    println!("\nTitre extrait : \"{}\"", titre);
    // t2 n'est plus utilisable ici (moved dans extraire_titre)

    // Statistiques
    let toutes: Vec<(String, u8, bool)> = vec![t1_done, t3, t4_done];
    let nb_total = toutes.len();
    let nb_completes = toutes.iter().filter(|t| t.2).count();
    println!("\n📊 Statistiques :");
    println!("   Total    : {}", nb_total);
    println!("   Complètes: {}", nb_completes);
    println!("   En cours : {}", nb_total - nb_completes);
}

// ============================================================
// Tests unitaires
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creer_tache_incomplete() {
        let t = creer_tache(String::from("Test"), 3);
        assert_eq!(t.0, "Test");
        assert_eq!(t.1, 3);
        assert!(!t.2); // doit être incomplète
    }

    #[test]
    fn test_marquer_complete() {
        let t = creer_tache(String::from("Tâche"), 2);
        let t_done = marquer_complete(t);
        assert!(t_done.2); // doit être complète
        assert_eq!(t_done.0, "Tâche"); // titre préservé
        assert_eq!(t_done.1, 2);       // priorité préservée
    }

    #[test]
    fn test_extraire_titre() {
        let t = creer_tache(String::from("Mon titre"), 1);
        let titre = extraire_titre(t);
        assert_eq!(titre, "Mon titre");
    }

    #[test]
    fn test_haute_priorite() {
        let t_haute = creer_tache(String::from("Urgente"), 5);
        let t_basse = creer_tache(String::from("Normale"), 2);
        assert!(est_haute_priorite(&t_haute));
        assert!(!est_haute_priorite(&t_basse));
    }

    #[test]
    fn test_somme_sans_move() {
        let v = vec![10, 20, 30];
        let s = somme(&v);
        assert_eq!(s, 60);
        assert_eq!(v.len(), 3); // v encore valide
    }

    #[test]
    #[should_panic]
    fn test_priorite_invalide() {
        creer_tache(String::from("Invalide"), 6); // doit paniquer
    }
}

