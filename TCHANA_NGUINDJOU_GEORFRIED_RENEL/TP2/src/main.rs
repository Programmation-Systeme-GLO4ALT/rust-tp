// ============================================================
// TP 2 — Gestion Mémoire Manuelle
// Séance 2 : Ownership & Borrowing — Partie I
// ============================================================
// Ce fichier contient :
//   - Partie A : correction des programmes bugués (ownership)
//   - Partie B : gestionnaire de tâches sans références
// ============================================================

// ============================================================
// PARTIE A — Programmes corrigés
// ============================================================

/// Programme 1 corrigé : on clone v pour garder les deux valides.
#[allow(dead_code)]
fn programme1_corrige() {
    let v = vec![1, 2, 3];
    let v2 = v.clone(); // Clone explicite : nouvelle allocation heap
    println!("Longueur de v  : {}", v.len());  // OK
    println!("Longueur de v2 : {}", v2.len()); // OK
}

/// Programme 2 corrigé : on passe une référence &[i32] au lieu de Vec<i32>.
/// &[i32] est une slice : plus flexible que &Vec<i32>.
fn somme(v: &[i32]) -> i32 {
    v.iter().sum()
}

#[allow(dead_code)]
fn programme2_corrige() {
    let nombres = vec![1, 2, 3, 4, 5];
    let s = somme(&nombres); // &nombres = référence immuable, pas de move
    // nombres est toujours valide !
    println!("Somme : {}, Vecteur : {:?}", s, nombres);
}

// ============================================================
// PARTIE B — Gestionnaire de tâches (sans références &)
// ============================================================

// Type alias : (titre, priorité 1-5, est_complète)
type Tache = (String, u8, bool);

/// Crée une nouvelle tâche. Prend ownership de 'titre'.
fn creer_tache(titre: String, priorite: u8) -> Tache {
    (titre, priorite, false)
}

/// Affiche une tâche. Consomme la tâche (ownership transféré).
fn afficher_tache(tache: Tache) {
    let statut = if tache.2 { "✅ Complète" } else { "⏳ En cours" };
    let priorite_label = match tache.1 {
        1 => "Critique",
        2 => "Haute",
        3 => "Normale",
        4 => "Faible",
        _ => "Inconnue",
    };
    println!("[{}] {} — {}", priorite_label, tache.0, statut);
}

/// Marque une tâche comme complète.
/// Pattern : consume & rebuild (pas de &mut car pas de références autorisées).
fn marquer_complete(tache: Tache) -> Tache {
    (tache.0, tache.1, true)
}

/// Extrait le titre d'une tâche (consomme la tâche, retourne le titre).
fn extraire_titre(tache: Tache) -> String {
    tache.0 // ownership du String transféré à l'appelant
}

/// Modifie la priorité d'une tâche. Consume & rebuild.
fn changer_priorite(tache: Tache, nouvelle_priorite: u8) -> Tache {
    (tache.0, nouvelle_priorite, tache.2)
}

fn main() {
    println!("=== Démonstration Ownership (TP 2) ===\n");

    // ── Partie A ──────────────────────────────────────────────
    println!("--- Partie A : Programmes corrigés ---");
    programme1_corrige();
    programme2_corrige();
    println!();

    // ── Partie B ──────────────────────────────────────────────
    println!("--- Partie B : Gestionnaire de tâches ---\n");

    // Création
    let t1 = creer_tache(String::from("Apprendre Rust"), 1);
    let t2 = creer_tache(String::from("Écrire des tests unitaires"), 2);
    let t3 = creer_tache(String::from("Déployer l'application"), 3);
    let t4 = creer_tache(String::from("Documenter l'API"), 4);

    // Modification (t1 est consumed, t1_done prend ownership)
    let t1_done = marquer_complete(t1);
    // t1 n'existe plus ici — le compilateur l'interdit

    // Changer la priorité de t2 (t2 consumed, t2_updated prend ownership)
    let t2_updated = changer_priorite(t2, 1);

    // Affichage (chaque tâche est consumed par afficher_tache)
    println!("Liste des tâches :");
    afficher_tache(t1_done);
    afficher_tache(t2_updated);
    afficher_tache(t3);
    // t4 : on extrait d'abord le titre
    let titre_t4 = extraire_titre(t4);
    println!("Titre extrait de t4 : '{}'", titre_t4);
    // t4 n'existe plus ici, mais titre_t4 (String) est toujours valide

    // ── Gestion d'une liste (Vec) ─────────────────────────────
    println!("\n--- Liste complète de tâches ---");
    let mut liste: Vec<Tache> = Vec::new();

    liste.push(creer_tache(String::from("Tâche A"), 2));
    liste.push(creer_tache(String::from("Tâche B"), 3));
    liste.push(creer_tache(String::from("Tâche C"), 1));
    liste.push(creer_tache(String::from("Tâche D"), 2));

    // Marquer les tâches de priorité 1 comme complètes (par transformation)
    let liste: Vec<Tache> = liste
        .into_iter()
        .map(|t| if t.1 == 1 { marquer_complete(t) } else { t })
        .collect();

    for tache in liste {
        afficher_tache(tache);
    }
}

// ============================================================
// Tests unitaires
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creer_tache() {
        let t = creer_tache(String::from("Test"), 3);
        assert_eq!(t.0, "Test");
        assert_eq!(t.1, 3);
        assert_eq!(t.2, false);
    }

    #[test]
    fn test_marquer_complete() {
        let t = creer_tache(String::from("Test"), 2);
        let t = marquer_complete(t);
        assert!(t.2);
        assert_eq!(t.0, "Test"); // titre préservé
    }

    #[test]
    fn test_extraire_titre() {
        let t = creer_tache(String::from("Mon titre"), 1);
        let titre = extraire_titre(t);
        assert_eq!(titre, "Mon titre");
    }

    #[test]
    fn test_changer_priorite() {
        let t = creer_tache(String::from("Test"), 3);
        let t = changer_priorite(t, 1);
        assert_eq!(t.1, 1);
    }

    #[test]
    fn test_somme_slice() {
        assert_eq!(somme(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(somme(&[]), 0);
        assert_eq!(somme(&[42]), 42);
    }
}
