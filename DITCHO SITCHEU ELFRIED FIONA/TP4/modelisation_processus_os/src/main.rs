// =================================================================
// TP 4 — Modélisation de Processus OS  (Séance 4)
// =================================================================
// Thèmes : structs, enums avec données, pattern matching,
//          Option<T>, Result<T,E>, blocs impl
// cargo new tp4_processus && cp tp4_processus.rs tp4_processus/src/main.rs
// cd tp4_processus && cargo test
// =================================================================

// -----------------------------------------------------------------
//  Types de données
// -----------------------------------------------------------------

#[derive(Debug, Clone)]
enum EtatProcessus {
    Prêt,
    EnExécution { cpu_id: u8 },
    Bloqué { raison: String },
    Terminé { code_retour: i32 },
    Zombie,
}

#[derive(Debug, Clone)]
enum Priorité {
    TrèsFaible,
    Faible,
    Normale,
    Haute,
    TrèsHaute,
    TempsRéel(u8), // niveau 0-99
}

impl Priorité {
    fn valeur(&self) -> u8 {
        match self {
            Priorité::TrèsFaible    => 1,
            Priorité::Faible        => 2,
            Priorité::Normale       => 3,
            Priorité::Haute         => 4,
            Priorité::TrèsHaute     => 5,
            Priorité::TempsRéel(n)  => *n,
        }
    }
}

impl std::fmt::Display for Priorité {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priorité::TrèsFaible    => write!(f, "TrèsFaible"),
            Priorité::Faible        => write!(f, "Faible"),
            Priorité::Normale       => write!(f, "Normale"),
            Priorité::Haute         => write!(f, "Haute"),
            Priorité::TrèsHaute     => write!(f, "TrèsHaute"),
            Priorité::TempsRéel(n)  => write!(f, "TempsRéel({})", n),
        }
    }
}

impl std::fmt::Display for EtatProcessus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EtatProcessus::Prêt                    => write!(f, "Prêt"),
            EtatProcessus::EnExécution { cpu_id }  => write!(f, "EnExéc[CPU{}]", cpu_id),
            EtatProcessus::Bloqué { raison }       => write!(f, "Bloqué({})", raison),
            EtatProcessus::Terminé { code_retour } => write!(f, "Terminé({})", code_retour),
            EtatProcessus::Zombie                  => write!(f, "Zombie"),
        }
    }
}

#[derive(Debug)]
struct Processus {
    pid: u32,
    nom: String,
    état: EtatProcessus,
    priorité: Priorité,
    mémoire_ko: u64,
    pid_parent: Option<u32>,
}

#[derive(Debug)]
struct GestionnaireProcessus {
    processus: Vec<Processus>,
    prochain_pid: u32,
}

// -----------------------------------------------------------------
//  Implémentation du gestionnaire
// -----------------------------------------------------------------

impl GestionnaireProcessus {
    /// Crée un gestionnaire vide.
    fn nouveau() -> Self {
        GestionnaireProcessus {
            processus: Vec::new(),
            prochain_pid: 1,
        }
    }

    /// Ajoute un processus et retourne son PID.
    fn créer_processus(
        &mut self,
        nom: String,
        priorité: Priorité,
        mémoire_ko: u64,
        pid_parent: Option<u32>,
    ) -> u32 {
        let pid = self.prochain_pid;
        self.prochain_pid += 1;

        self.processus.push(Processus {
            pid,
            nom,
            état: EtatProcessus::Prêt, // tout nouveau processus démarre Prêt
            priorité,
            mémoire_ko,
            pid_parent,
        });

        pid
    }

    /// Recherche en lecture un processus par PID.
    fn trouver(&self, pid: u32) -> Option<&Processus> {
        self.processus.iter().find(|p| p.pid == pid)
    }

    /// Recherche en écriture un processus par PID.
    fn trouver_mut(&mut self, pid: u32) -> Option<&mut Processus> {
        self.processus.iter_mut().find(|p| p.pid == pid)
    }

    /// Change l'état d'un processus existant.
    /// Erreur si PID introuvable ou si le processus est déjà Terminé/Zombie.
    fn changer_état(
        &mut self,
        pid: u32,
        nouvel_état: EtatProcessus,
    ) -> Result<(), String> {
        match self.trouver_mut(pid) {
            Some(proc) => {
                match &proc.état {
                    EtatProcessus::Terminé { code_retour } => {
                        return Err(format!(
                            "PID {} est déjà Terminé (code {}), impossible de changer son état",
                            pid, code_retour
                        ))
                    }
                    EtatProcessus::Zombie => {
                        return Err(format!("PID {} est Zombie, état immuable", pid))
                    }
                    _ => {}
                }
                proc.état = nouvel_état;
                Ok(())
            }
            None => Err(format!("PID {} introuvable", pid)),
        }
    }

    /// Somme de la mémoire des processus actifs (non Terminés ni Zombie).
    fn mémoire_totale_utilisée(&self) -> u64 {
        self.processus
            .iter()
            .filter(|p| !matches!(
                p.état,
                EtatProcessus::Terminé { .. } | EtatProcessus::Zombie
            ))
            .map(|p| p.mémoire_ko)
            .sum()
    }

    /// Retourne tous les processus dont le discriminant d'état correspond.
    /// Utilise std::mem::discriminant pour comparer les variantes sans
    /// se soucier des données internes.
    fn processus_par_état(&self, état: &EtatProcessus) -> Vec<&Processus> {
        self.processus
            .iter()
            .filter(|p| {
                std::mem::discriminant(&p.état) == std::mem::discriminant(état)
            })
            .collect()
    }

    /// Passe un processus en Terminé { code_retour: 0 }.
    /// Retourne le code de retour ou une erreur.
    fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {
        match self.trouver_mut(pid) {
            Some(proc) => match proc.état {
                EtatProcessus::Terminé { code_retour } => Err(format!(
                    "PID {} est déjà terminé avec code {}", pid, code_retour
                )),
                _ => {
                    proc.état = EtatProcessus::Terminé { code_retour: 0 };
                    Ok(0)
                }
            },
            None => Err(format!("PID {} introuvable", pid)),
        }
    }

    /// Affiche un tableau récapitulatif de tous les processus.
    fn afficher_résumé(&self) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║                  GESTIONNAIRE DE PROCESSUS                  ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!("{:<5} {:<12} {:<20} {:<12} {:<9} {:<5}",
            "PID", "NOM", "ÉTAT", "PRIORITÉ", "MEM(ko)", "PPID");
        println!("{}", "─".repeat(65));

        let mut liste: Vec<&Processus> = self.processus.iter().collect();
        // Tri par PID pour un affichage stable
        liste.sort_by_key(|p| p.pid);

        for proc in &liste {
            let ppid = proc.pid_parent
                .map(|p| p.to_string())
                .unwrap_or_else(|| "—".to_string());

            println!("{:<5} {:<12} {:<20} {:<12} {:<9} {:<5}",
                proc.pid,
                proc.nom,
                proc.état.to_string(),
                proc.priorité.to_string(),
                proc.mémoire_ko,
                ppid,
            );
        }

        println!("{}", "─".repeat(65));
        println!("  Total processus : {} | Mémoire active : {} ko",
            self.processus.len(),
            self.mémoire_totale_utilisée()
        );
    }
}

// -----------------------------------------------------------------
//  Programme principal (scénario Unix simplifié)
// -----------------------------------------------------------------

fn main() {
    println!("=== TP 4 — Modélisation de Processus OS ===\n");

    let mut gp = GestionnaireProcessus::nouveau();

    // --- Créer la hiérarchie de processus ---
    let init = gp.créer_processus(
        String::from("init"),
        Priorité::Haute,
        1024,
        None, // pas de parent → processus racine
    );

    let bash = gp.créer_processus(
        String::from("bash"),
        Priorité::Normale,
        4096,
        Some(init),
    );

    let vim = gp.créer_processus(
        String::from("vim"),
        Priorité::Normale,
        8192,
        Some(bash),
    );

    let _rustc = gp.créer_processus(
        String::from("rustc"),
        Priorité::TrèsHaute,
        131072,
        Some(bash),
    );

    let rt = gp.créer_processus(
        String::from("rt_sched"),
        Priorité::TempsRéel(80),
        256,
        Some(init),
    );

    // --- Changer les états ---
    gp.changer_état(init, EtatProcessus::EnExécution { cpu_id: 0 }).unwrap();
    gp.changer_état(bash, EtatProcessus::EnExécution { cpu_id: 1 }).unwrap();
    gp.changer_état(vim, EtatProcessus::Bloqué {
        raison: String::from("attente I/O"),
    }).unwrap();
    gp.changer_état(rt, EtatProcessus::EnExécution { cpu_id: 0 }).unwrap();

    gp.afficher_résumé();

    // --- Tuer bash ---
    println!();
    match gp.tuer_processus(bash) {
        Ok(code) => println!("✓ bash (PID {}) terminé avec code {}", bash, code),
        Err(e)   => eprintln!("✗ {}", e),
    }

    // --- Cas d'erreur : PID inexistant ---
    match gp.tuer_processus(999) {
        Ok(_)  => {},
        Err(e) => println!("✗ Erreur attendue : {}", e),
    }

    // --- Cas d'erreur : changer l'état d'un terminé ---
    match gp.changer_état(bash, EtatProcessus::Prêt) {
        Ok(_)  => {},
        Err(e) => println!("✗ Erreur attendue : {}", e),
    }

    // --- Liste des processus en exécution ---
    let en_exec = gp.processus_par_état(&EtatProcessus::EnExécution { cpu_id: 0 });
    println!("\nProcessus en exécution ({}) :", en_exec.len());
    for p in en_exec {
        println!("  → [CPU{}] {} (PID {}, {} ko)",
            match &p.état {
                EtatProcessus::EnExécution { cpu_id } => *cpu_id,
                _ => 0,
            },
            p.nom, p.pid, p.mémoire_ko
        );
    }

    // --- Résumé final ---
    gp.afficher_résumé();

    // --- Recherche par PID ---
    println!();
    match gp.trouver(vim) {
        Some(p) => println!("Processus vim (PID {}) : état = {}", p.pid, p.état),
        None    => println!("PID introuvable"),
    }
}

// -----------------------------------------------------------------
//  Tests
// -----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn préparer() -> (GestionnaireProcessus, u32, u32) {
        let mut gp = GestionnaireProcessus::nouveau();
        let init = gp.créer_processus(String::from("init"), Priorité::Haute, 1024, None);
        let bash = gp.créer_processus(String::from("bash"), Priorité::Normale, 4096, Some(init));
        (gp, init, bash)
    }

    #[test]
    fn test_créer_pid_auto() {
        let (gp, init, bash) = préparer();
        assert_eq!(init, 1);
        assert_eq!(bash, 2);
        assert_eq!(gp.processus.len(), 2);
    }

    #[test]
    fn test_trouver() {
        let (gp, init, _) = préparer();
        assert!(gp.trouver(init).is_some());
        assert!(gp.trouver(999).is_none());
        assert_eq!(gp.trouver(init).unwrap().nom, "init");
    }

    #[test]
    fn test_changer_état_ok() {
        let (mut gp, init, _) = préparer();
        assert!(gp.changer_état(init, EtatProcessus::EnExécution { cpu_id: 0 }).is_ok());
        assert!(matches!(
            gp.trouver(init).unwrap().état,
            EtatProcessus::EnExécution { .. }
        ));
    }

    #[test]
    fn test_changer_état_pid_inexistant() {
        let (mut gp, _, _) = préparer();
        assert!(gp.changer_état(999, EtatProcessus::Prêt).is_err());
    }

    #[test]
    fn test_changer_état_terminé_interdit() {
        let (mut gp, _, bash) = préparer();
        gp.tuer_processus(bash).unwrap();
        assert!(gp.changer_état(bash, EtatProcessus::Prêt).is_err());
    }

    #[test]
    fn test_tuer_processus() {
        let (mut gp, _, bash) = préparer();
        assert_eq!(gp.tuer_processus(bash), Ok(0));
    }

    #[test]
    fn test_tuer_deux_fois() {
        let (mut gp, _, bash) = préparer();
        gp.tuer_processus(bash).unwrap();
        assert!(gp.tuer_processus(bash).is_err());
    }

    #[test]
    fn test_tuer_pid_inexistant() {
        let (mut gp, _, _) = préparer();
        assert!(gp.tuer_processus(999).is_err());
    }

    #[test]
    fn test_mémoire_totale() {
        let (mut gp, init, bash) = préparer();
        assert_eq!(gp.mémoire_totale_utilisée(), 1024 + 4096);

        gp.tuer_processus(bash).unwrap();
        assert_eq!(gp.mémoire_totale_utilisée(), 1024);

        gp.tuer_processus(init).unwrap();
        assert_eq!(gp.mémoire_totale_utilisée(), 0);
    }

    #[test]
    fn test_processus_par_état() {
        let (mut gp, init, bash) = préparer();
        gp.changer_état(init, EtatProcessus::EnExécution { cpu_id: 0 }).unwrap();
        gp.changer_état(bash, EtatProcessus::EnExécution { cpu_id: 1 }).unwrap();

        let en_exec = gp.processus_par_état(&EtatProcessus::EnExécution { cpu_id: 0 });
        assert_eq!(en_exec.len(), 2);

        let prêts = gp.processus_par_état(&EtatProcessus::Prêt);
        assert_eq!(prêts.len(), 0);
    }

    #[test]
    fn test_priorité_valeur() {
        assert_eq!(Priorité::TrèsFaible.valeur(), 1);
        assert_eq!(Priorité::Normale.valeur(), 3);
        assert_eq!(Priorité::TempsRéel(99).valeur(), 99);
    }
}
