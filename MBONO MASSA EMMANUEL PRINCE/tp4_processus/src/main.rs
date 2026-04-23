// ============================================================
// TP 4 — Modélisation de Processus OS
// Programmation Système avec Rust — GL4 ENSPD 2025-2026
// ============================================================
//
// Modélise un gestionnaire de processus Unix simplifié.
// Met en pratique : structs, enums, pattern matching,
//                   Option<T>, Result<T,E>, méthodes impl.
// ============================================================

// ============================================================
// Définition des types
// ============================================================

#[derive(Debug, Clone, PartialEq)]
enum EtatProcessus {
    Prêt,
    EnExécution { cpu_id: u8 },
    Bloqué { raison: String },
    Terminé { code_retour: i32 },
    Zombie,
}

#[derive(Debug, Clone, PartialEq)]
enum Priorité {
    TrèsFaible,
    Faible,
    Normale,
    Haute,
    TrèsHaute,
    TempsRéel(u8), // niveau 0-99
}

impl Priorité {
    // Valeur numérique pour trier
    fn valeur(&self) -> u8 {
        match self {
            Priorité::TrèsFaible  => 1,
            Priorité::Faible      => 2,
            Priorité::Normale     => 3,
            Priorité::Haute       => 4,
            Priorité::TrèsHaute   => 5,
            Priorité::TempsRéel(n) => 6 + n,
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

// ============================================================
// Implémentation du gestionnaire de processus
// ============================================================
impl GestionnaireProcessus {
    // Crée un gestionnaire vide (PID commence à 1)
    fn nouveau() -> Self {
        GestionnaireProcessus {
            processus: Vec::new(),
            prochain_pid: 1,
        }
    }

    // Crée un processus et l'ajoute au gestionnaire
    // Retourne le PID attribué
    fn créer_processus(
        &mut self,
        nom: String,
        priorité: Priorité,
        mémoire_ko: u64,
        pid_parent: Option<u32>,
    ) -> u32 {
        // Vérifier que le parent existe (si spécifié)
        if let Some(ppid) = pid_parent {
            if self.trouver(ppid).is_none() {
                panic!("PID parent {} introuvable", ppid);
            }
        }

        let pid = self.prochain_pid;
        self.prochain_pid += 1;

        self.processus.push(Processus {
            pid,
            nom,
            état: EtatProcessus::Prêt,
            priorité,
            mémoire_ko,
            pid_parent,
        });

        pid
    }

    // Trouve un processus par PID — retourne une référence optionnelle
    fn trouver(&self, pid: u32) -> Option<&Processus> {
        self.processus.iter().find(|p| p.pid == pid)
    }

    // Trouve un processus par PID (mutable)
    fn trouver_mut(&mut self, pid: u32) -> Option<&mut Processus> {
        self.processus.iter_mut().find(|p| p.pid == pid)
    }

    // Change l'état d'un processus
    // Retourne Ok(()) ou Err(message) si PID introuvable
    fn changer_état(
        &mut self,
        pid: u32,
        nouvel_état: EtatProcessus,
    ) -> Result<(), String> {
        match self.trouver_mut(pid) {
            Some(proc) => {
                proc.état = nouvel_état;
                Ok(())
            }
            None => Err(format!("Processus PID {} introuvable", pid)),
        }
    }

    // Calcule la mémoire totale utilisée par tous les processus actifs
    fn mémoire_totale_utilisée(&self) -> u64 {
        self.processus
            .iter()
            .filter(|p| !matches!(p.état, EtatProcessus::Terminé { .. } | EtatProcessus::Zombie))
            .map(|p| p.mémoire_ko)
            .sum()
    }

    // Retourne tous les processus dans un état donné
    fn processus_par_état(&self, état: &EtatProcessus) -> Vec<&Processus> {
        self.processus
            .iter()
            .filter(|p| {
                // Comparaison sur le discriminant de l'enum (pas les données internes)
                std::mem::discriminant(&p.état) == std::mem::discriminant(état)
            })
            .collect()
    }

    // Tue un processus (passe à Terminé { code_retour: 0 })
    // Retourne le code de retour ou une Err si PID introuvable
    fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {
        match self.trouver_mut(pid) {
            Some(proc) => {
                proc.état = EtatProcessus::Terminé { code_retour: 0 };
                Ok(0)
            }
            None => Err(format!("Impossible de tuer PID {} : introuvable", pid)),
        }
    }

    // Retourne les processus triés par priorité décroissante
    fn processus_par_priorité(&self) -> Vec<&Processus> {
        let mut liste: Vec<&Processus> = self.processus.iter().collect();
        liste.sort_by(|a, b| b.priorité.valeur().cmp(&a.priorité.valeur()));
        liste
    }

    // Affiche un résumé complet du gestionnaire
    fn afficher_résumé(&self) {
        println!("\n┌─────────────────────────────────────────────────────────────────────┐");
        println!("│  GESTIONNAIRE DE PROCESSUS — {} processus                         │", self.processus.len());
        println!("├──────┬───────────────────────────┬──────────────────┬──────────────┤");
        println!("│ PID  │ NOM                       │ ÉTAT             │  MÉMOIRE(Ko) │");
        println!("├──────┼───────────────────────────┼──────────────────┼──────────────┤");

        for proc in self.processus_par_priorité() {
            let état_str = match &proc.état {
                EtatProcessus::Prêt                     => "Prêt            ".to_string(),
                EtatProcessus::EnExécution { cpu_id }   => format!("En exec (CPU {})", cpu_id),
                EtatProcessus::Bloqué { raison }        => format!("Bloqué: {:.8}", raison),
                EtatProcessus::Terminé { code_retour }  => format!("Terminé ({})", code_retour),
                EtatProcessus::Zombie                   => "Zombie          ".to_string(),
            };
            let parent = proc.pid_parent
                .map(|p| format!("(ppid={})", p))
                .unwrap_or_default();
            println!(
                "│ {:>4} │ {:<25} │ {:<16} │ {:>12} │  {}",
                proc.pid,
                proc.nom,
                état_str,
                proc.mémoire_ko,
                parent
            );
        }

        println!("└──────┴───────────────────────────┴──────────────────┴──────────────┘");
        println!("  Mémoire totale utilisée : {} Ko", self.mémoire_totale_utilisée());
    }
}

// ============================================================
// Programme principal — simulation d'un système Unix minimal
// ============================================================
fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║   TP4 — Modélisation de Processus OS        ║");
    println!("╚══════════════════════════════════════════════╝");

    let mut gp = GestionnaireProcessus::nouveau();

    // Création de processus
    let pid_init  = gp.créer_processus("init".into(),    Priorité::TrèsHaute,   4096, None);
    let pid_bash  = gp.créer_processus("bash".into(),    Priorité::Normale,     8192, Some(pid_init));
    let pid_vim   = gp.créer_processus("vim".into(),     Priorité::Faible,      16384, Some(pid_bash));
    let pid_cargo = gp.créer_processus("cargo build".into(), Priorité::Haute,   65536, Some(pid_bash));
    let pid_rt    = gp.créer_processus("audio-daemon".into(), Priorité::TempsRéel(50), 2048, Some(pid_init));

    println!("\n--- État initial ---");
    gp.afficher_résumé();

    // Transitions d'état
    gp.changer_état(pid_init,  EtatProcessus::EnExécution { cpu_id: 0 }).unwrap();
    gp.changer_état(pid_bash,  EtatProcessus::EnExécution { cpu_id: 1 }).unwrap();
    gp.changer_état(pid_cargo, EtatProcessus::EnExécution { cpu_id: 2 }).unwrap();
    gp.changer_état(pid_vim,   EtatProcessus::Bloqué { raison: "attente I/O".into() }).unwrap();
    gp.changer_état(pid_rt,    EtatProcessus::EnExécution { cpu_id: 3 }).unwrap();

    println!("\n--- Après démarrage ---");
    gp.afficher_résumé();

    // Tuer vim
    match gp.tuer_processus(pid_vim) {
        Ok(code) => println!("\n  ✅ vim (PID {}) tué. Code de retour : {}", pid_vim, code),
        Err(e)   => println!("\n  ❌ Erreur : {}", e),
    }

    // Tuer un PID inexistant
    match gp.tuer_processus(999) {
        Ok(_)  => println!("  ✅ Tué"),
        Err(e) => println!("  ❌ {}", e),
    }

    // Statistiques par état
    println!("\n--- Processus en exécution ---");
    for p in gp.processus_par_état(&EtatProcessus::EnExécution { cpu_id: 0 }) {
        println!("  PID {:>4} | {} | {} Ko", p.pid, p.nom, p.mémoire_ko);
    }

    // Processus par PID
    println!("\n--- Recherche PID {} ---", pid_cargo);
    match gp.trouver(pid_cargo) {
        Some(p) => println!("  Trouvé : {} | Priorité : {:?} | État : {:?}", p.nom, p.priorité, p.état),
        None    => println!("  Introuvable"),
    }

    println!("\n--- État final ---");
    gp.afficher_résumé();
}

// ============================================================
// Tests unitaires
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> GestionnaireProcessus {
        let mut gp = GestionnaireProcessus::nouveau();
        gp.créer_processus("init".into(), Priorité::TrèsHaute, 4096, None);
        gp.créer_processus("bash".into(), Priorité::Normale, 8192, Some(1));
        gp.créer_processus("vim".into(),  Priorité::Faible,  2048, Some(2));
        gp
    }

    #[test]
    fn test_creer_processus_pid_incremental() {
        let mut gp = GestionnaireProcessus::nouveau();
        let p1 = gp.créer_processus("a".into(), Priorité::Normale, 0, None);
        let p2 = gp.créer_processus("b".into(), Priorité::Normale, 0, None);
        assert_eq!(p1, 1);
        assert_eq!(p2, 2);
    }

    #[test]
    fn test_processus_etat_initial_pret() {
        let gp = setup();
        let p = gp.trouver(1).unwrap();
        assert_eq!(p.état, EtatProcessus::Prêt);
    }

    #[test]
    fn test_changer_etat_ok() {
        let mut gp = setup();
        let res = gp.changer_état(1, EtatProcessus::EnExécution { cpu_id: 0 });
        assert!(res.is_ok());
        assert_eq!(gp.trouver(1).unwrap().état, EtatProcessus::EnExécution { cpu_id: 0 });
    }

    #[test]
    fn test_changer_etat_pid_inexistant() {
        let mut gp = setup();
        let res = gp.changer_état(999, EtatProcessus::Zombie);
        assert!(res.is_err());
    }

    #[test]
    fn test_tuer_processus() {
        let mut gp = setup();
        let code = gp.tuer_processus(3).unwrap();
        assert_eq!(code, 0);
        assert!(matches!(gp.trouver(3).unwrap().état, EtatProcessus::Terminé { code_retour: 0 }));
    }

    #[test]
    fn test_tuer_pid_inexistant() {
        let mut gp = setup();
        assert!(gp.tuer_processus(999).is_err());
    }

    #[test]
    fn test_memoire_totale_exclut_termines() {
        let mut gp = setup();
        // init=4096, bash=8192, vim=2048 → total=14336
        assert_eq!(gp.mémoire_totale_utilisée(), 14336);
        // Tuer vim → total=12288
        gp.tuer_processus(3).unwrap();
        assert_eq!(gp.mémoire_totale_utilisée(), 12288);
    }

    #[test]
    fn test_processus_par_etat() {
        let mut gp = setup();
        gp.changer_état(1, EtatProcessus::EnExécution { cpu_id: 0 }).unwrap();
        gp.changer_état(2, EtatProcessus::EnExécution { cpu_id: 1 }).unwrap();
        let en_exec = gp.processus_par_état(&EtatProcessus::EnExécution { cpu_id: 0 });
        assert_eq!(en_exec.len(), 2);
    }

    #[test]
    fn test_trouver_existant() {
        let gp = setup();
        assert!(gp.trouver(2).is_some());
    }

    #[test]
    fn test_trouver_inexistant() {
        let gp = setup();
        assert!(gp.trouver(999).is_none());
    }
}

