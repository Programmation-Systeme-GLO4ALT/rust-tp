// ============================================================
// TP 4 — Modélisation de Processus OS
// Séance 4 : Types Système & Structures
// ============================================================
// Concepts couverts :
//   - Structs nommées et blocs impl
//   - Enums avec données (variantes struct et tuple)
//   - Pattern matching exhaustif
//   - Option<T> pour l'absence de valeur
//   - Result<T, E> pour la gestion d'erreurs
// ============================================================

// ============================================================
// Types de données
// ============================================================

#[derive(Debug, Clone, PartialEq)]
pub enum EtatProcessus {
    Pret,
    EnExecution { cpu_id: u8 },
    Bloque { raison: String },
    Termine { code_retour: i32 },
    Zombie,
}

#[derive(Debug, Clone)]
pub enum Priorite {
    TresFaible,
    Faible,
    Normale,
    Haute,
    TresHaute,
    TempsReel(u8), // niveau 0-99
}

impl Priorite {
    /// Retourne la valeur numérique de la priorité (pour tri/comparaison).
    pub fn valeur(&self) -> u8 {
        match self {
            Priorite::TresFaible   => 1,
            Priorite::Faible       => 2,
            Priorite::Normale      => 3,
            Priorite::Haute        => 4,
            Priorite::TresHaute    => 5,
            Priorite::TempsReel(n) => *n,
        }
    }

    pub fn label(&self) -> String {
        match self {
            Priorite::TresFaible      => "TrèsFaible".to_string(),
            Priorite::Faible          => "Faible".to_string(),
            Priorite::Normale         => "Normale".to_string(),
            Priorite::Haute           => "Haute".to_string(),
            Priorite::TresHaute       => "TrèsHaute".to_string(),
            Priorite::TempsReel(n)    => format!("RT({})", n),
        }
    }
}

#[derive(Debug)]
pub struct Processus {
    pub pid: u32,
    pub nom: String,
    pub etat: EtatProcessus,
    pub priorite: Priorite,
    pub memoire_ko: u64,
    pub pid_parent: Option<u32>, // None = processus racine (init)
}

impl Processus {
    /// Retourne une description textuelle de l'état courant.
    pub fn etat_label(&self) -> String {
        match &self.etat {
            EtatProcessus::Pret                    => "Prêt".to_string(),
            EtatProcessus::EnExecution { cpu_id }  => format!("CPU #{}", cpu_id),
            EtatProcessus::Bloque { raison }        => format!("Bloqué ({})", raison),
            EtatProcessus::Termine { code_retour }  => format!("Terminé [{}]", code_retour),
            EtatProcessus::Zombie                  => "Zombie".to_string(),
        }
    }
}

// ============================================================
// Gestionnaire de processus
// ============================================================

#[derive(Debug)]
pub struct GestionnaireProcessus {
    processus: Vec<Processus>,
    prochain_pid: u32,
}

impl GestionnaireProcessus {
    /// Crée un nouveau gestionnaire vide. Les PIDs commencent à 1.
    pub fn nouveau() -> Self {
        GestionnaireProcessus {
            processus: Vec::new(),
            prochain_pid: 1,
        }
    }

    /// Crée un nouveau processus, l'ajoute à la table et retourne son PID.
    pub fn creer_processus(
        &mut self,
        nom: String,
        priorite: Priorite,
        memoire_ko: u64,
        pid_parent: Option<u32>,
    ) -> u32 {
        let pid = self.prochain_pid;
        self.prochain_pid += 1;

        self.processus.push(Processus {
            pid,
            nom,
            etat: EtatProcessus::Pret, // tout processus démarre à l'état Prêt
            priorite,
            memoire_ko,
            pid_parent,
        });

        pid
    }

    /// Recherche un processus par PID.
    /// Retourne None si le PID n'existe pas.
    pub fn trouver(&self, pid: u32) -> Option<&Processus> {
        self.processus.iter().find(|p| p.pid == pid)
    }

    /// Recherche un processus mutable par PID (usage interne).
    fn trouver_mut(&mut self, pid: u32) -> Option<&mut Processus> {
        self.processus.iter_mut().find(|p| p.pid == pid)
    }

    /// Change l'état d'un processus.
    /// Retourne Err(String) si le PID est introuvable.
    pub fn changer_etat(
        &mut self,
        pid: u32,
        nouvel_etat: EtatProcessus,
    ) -> Result<(), String> {
        match self.trouver_mut(pid) {
            Some(proc) => {
                proc.etat = nouvel_etat;
                Ok(())
            }
            None => Err(format!("PID {} introuvable", pid)),
        }
    }

    /// Calcule la mémoire totale occupée par les processus actifs.
    /// Les processus Terminés et Zombies ne comptent pas.
    pub fn memoire_totale_utilisee(&self) -> u64 {
        self.processus
            .iter()
            .filter(|p| {
                !matches!(
                    p.etat,
                    EtatProcessus::Termine { .. } | EtatProcessus::Zombie
                )
            })
            .map(|p| p.memoire_ko)
            .sum()
    }

    /// Retourne les références vers tous les processus dans une variante d'état donnée.
    /// On compare les variantes (pas les données internes).
    pub fn processus_par_etat(&self, etat: &EtatProcessus) -> Vec<&Processus> {
        self.processus
            .iter()
            .filter(|p| {
                matches!(
                    (&p.etat, etat),
                    (EtatProcessus::Pret, EtatProcessus::Pret)
                    | (EtatProcessus::Zombie, EtatProcessus::Zombie)
                    | (EtatProcessus::EnExecution { .. }, EtatProcessus::EnExecution { .. })
                    | (EtatProcessus::Bloque { .. }, EtatProcessus::Bloque { .. })
                    | (EtatProcessus::Termine { .. }, EtatProcessus::Termine { .. })
                )
            })
            .collect()
    }

    /// Termine un processus (état → Terminé { code_retour: 0 }).
    /// Retourne le code de retour, ou Err si PID introuvable.
    pub fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {
        match self.trouver_mut(pid) {
            Some(proc) => {
                let code = 0i32; // convention Unix : 0 = terminaison normale
                proc.etat = EtatProcessus::Termine { code_retour: code };
                Ok(code)
            }
            None => Err(format!("Impossible de tuer PID {} : introuvable", pid)),
        }
    }

    /// Retourne le nombre total de processus enregistrés.
    pub fn nb_processus(&self) -> usize {
        self.processus.len()
    }

    /// Affiche un résumé tabulaire de tous les processus.
    pub fn afficher_resume(&self) {
        println!("\n╔══════╦════════════════════╦══════════════════════╦══════════╦══════════╗");
        println!(  "║ PID  ║ Nom                ║ État                 ║ Priorité ║ Mém (Ko) ║");
        println!(  "╠══════╬════════════════════╬══════════════════════╬══════════╬══════════╣");

        for p in &self.processus {
            let parent_info = match p.pid_parent {
                Some(ppid) => format!("(ppid:{})", ppid),
                None       => "  (root) ".to_string(),
            };
            println!(
                "║ {:>4} ║ {:<18} ║ {:<20} ║ {:<8} ║ {:>8} ║  {}",
                p.pid,
                trunc(&p.nom, 18),
                trunc(&p.etat_label(), 20),
                trunc(&p.priorite.label(), 8),
                p.memoire_ko,
                parent_info,
            );
        }

        println!("╚══════╩════════════════════╩══════════════════════╩══════════╩══════════╝");
        println!("  Processus total  : {}", self.processus.len());
        println!("  Mémoire active   : {} Ko\n", self.memoire_totale_utilisee());
    }
}

/// Tronque une chaîne à max_len caractères (pour l'affichage tableau).
fn trunc(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let mut r: String = s.chars().take(max_len - 1).collect();
        r.push('…');
        r
    }
}

// ============================================================
// Main
// ============================================================

fn main() {
    println!("=== Gestionnaire de Processus OS (TP 4) ===");

    let mut gp = GestionnaireProcessus::nouveau();

    // ── Création de la hiérarchie ─────────────────────────────
    let init = gp.creer_processus(
        "init".to_string(),
        Priorite::TresHaute,
        1024,
        None, // PID 1 — processus racine
    );
    println!("Créé : init (PID {})", init);

    let systemd = gp.creer_processus(
        "systemd".to_string(),
        Priorite::Haute,
        2048,
        Some(init),
    );

    let bash = gp.creer_processus(
        "bash".to_string(),
        Priorite::Normale,
        4096,
        Some(init),
    );

    let vim = gp.creer_processus(
        "vim".to_string(),
        Priorite::Normale,
        8192,
        Some(bash),
    );

    let cargo = gp.creer_processus(
        "cargo build".to_string(),
        Priorite::TempsReel(50),
        32768,
        Some(bash),
    );

    // ── Transitions d'état ────────────────────────────────────
    gp.changer_etat(bash, EtatProcessus::EnExecution { cpu_id: 0 })
        .expect("bash devrait exister");

    gp.changer_etat(vim, EtatProcessus::EnExecution { cpu_id: 1 })
        .expect("vim devrait exister");

    gp.changer_etat(cargo, EtatProcessus::Bloque {
        raison: "I/O disque".to_string(),
    }).expect("cargo devrait exister");

    gp.changer_etat(systemd, EtatProcessus::EnExecution { cpu_id: 0 })
        .expect("systemd devrait exister");

    // ── État initial ──────────────────────────────────────────
    println!("\n--- État initial du système ---");
    gp.afficher_resume();

    // ── Recherche ─────────────────────────────────────────────
    println!("--- Recherche de processus ---");
    match gp.trouver(bash) {
        Some(p) => println!("Trouvé PID {} : {} [{}]", p.pid, p.nom, p.etat_label()),
        None    => println!("PID {} introuvable", bash),
    }
    match gp.trouver(9999) {
        Some(p) => println!("Trouvé : {}", p.nom),
        None    => println!("PID 9999 introuvable (attendu)"),
    }

    // ── Terminer des processus ────────────────────────────────
    println!("\n--- Terminaison de processus ---");
    match gp.tuer_processus(vim) {
        Ok(code) => println!("vim (PID {}) terminé, code={}", vim, code),
        Err(e)   => eprintln!("Erreur : {}", e),
    }
    match gp.tuer_processus(cargo) {
        Ok(code) => println!("cargo (PID {}) terminé, code={}", cargo, code),
        Err(e)   => eprintln!("Erreur : {}", e),
    }
    // PID inexistant
    match gp.tuer_processus(9999) {
        Ok(_)  => println!("OK"),
        Err(e) => println!("Attendu — {}", e),
    }

    // ── État final ────────────────────────────────────────────
    println!("--- État final ---");
    gp.afficher_resume();

    // ── Statistiques par état ─────────────────────────────────
    println!("--- Statistiques par état ---");

    let en_exec = gp.processus_par_etat(&EtatProcessus::EnExecution { cpu_id: 0 });
    println!("En exécution ({}) :", en_exec.len());
    for p in &en_exec {
        println!("  PID {:>3} — {} [{}]", p.pid, p.nom, p.etat_label());
    }

    let termines = gp.processus_par_etat(&EtatProcessus::Termine { code_retour: 0 });
    println!("Terminés ({}) :", termines.len());
    for p in &termines {
        println!("  PID {:>3} — {}", p.pid, p.nom);
    }

    println!("\nMémoire totale active : {} Ko", gp.memoire_totale_utilisee());
}

// ============================================================
// Tests unitaires (cargo test)
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    /// Fixture : un gestionnaire pré-rempli avec init + bash.
    fn fixture() -> (GestionnaireProcessus, u32, u32) {
        let mut gp = GestionnaireProcessus::nouveau();
        let init = gp.creer_processus("init".to_string(), Priorite::Haute, 1024, None);
        let bash = gp.creer_processus("bash".to_string(), Priorite::Normale, 4096, Some(init));
        (gp, init, bash)
    }

    #[test]
    fn test_creation_pid_incremental() {
        let (gp, init, bash) = fixture();
        assert_eq!(init, 1);
        assert_eq!(bash, 2);
        assert_eq!(gp.nb_processus(), 2);
    }

    #[test]
    fn test_etat_initial_pret() {
        let (gp, init, _) = fixture();
        assert_eq!(gp.trouver(init).unwrap().etat, EtatProcessus::Pret);
    }

    #[test]
    fn test_trouver_existant() {
        let (gp, _, bash) = fixture();
        let p = gp.trouver(bash);
        assert!(p.is_some());
        assert_eq!(p.unwrap().nom, "bash");
    }

    #[test]
    fn test_trouver_inexistant() {
        let (gp, _, _) = fixture();
        assert!(gp.trouver(9999).is_none());
    }

    #[test]
    fn test_changer_etat_ok() {
        let (mut gp, _, bash) = fixture();
        let res = gp.changer_etat(bash, EtatProcessus::EnExecution { cpu_id: 2 });
        assert!(res.is_ok());
        assert!(matches!(
            gp.trouver(bash).unwrap().etat,
            EtatProcessus::EnExecution { cpu_id: 2 }
        ));
    }

    #[test]
    fn test_changer_etat_pid_inconnu() {
        let (mut gp, _, _) = fixture();
        let res = gp.changer_etat(9999, EtatProcessus::Zombie);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("9999"));
    }

    #[test]
    fn test_tuer_processus_ok() {
        let (mut gp, _, bash) = fixture();
        let res = gp.tuer_processus(bash);
        assert_eq!(res.unwrap(), 0);
        assert!(matches!(
            gp.trouver(bash).unwrap().etat,
            EtatProcessus::Termine { code_retour: 0 }
        ));
    }

    #[test]
    fn test_tuer_processus_inconnu() {
        let (mut gp, _, _) = fixture();
        assert!(gp.tuer_processus(9999).is_err());
    }

    #[test]
    fn test_memoire_exclut_termines() {
        let (mut gp, _, bash) = fixture();
        let avant = gp.memoire_totale_utilisee();
        assert_eq!(avant, 1024 + 4096);

        gp.tuer_processus(bash).unwrap();
        assert_eq!(gp.memoire_totale_utilisee(), 1024); // bash exclu
    }

    #[test]
    fn test_processus_par_etat() {
        let (mut gp, _, bash) = fixture();
        gp.changer_etat(bash, EtatProcessus::EnExecution { cpu_id: 0 }).unwrap();

        let prets = gp.processus_par_etat(&EtatProcessus::Pret);
        assert_eq!(prets.len(), 1); // seulement init

        let en_exec = gp.processus_par_etat(&EtatProcessus::EnExecution { cpu_id: 0 });
        assert_eq!(en_exec.len(), 1); // seulement bash
    }

    #[test]
    fn test_pid_parent_none() {
        let (gp, init, _) = fixture();
        assert!(gp.trouver(init).unwrap().pid_parent.is_none());
    }

    #[test]
    fn test_pid_parent_some() {
        let (gp, init, bash) = fixture();
        assert_eq!(gp.trouver(bash).unwrap().pid_parent, Some(init));
    }

    #[test]
    fn test_priorite_valeur() {
        assert!(Priorite::TresHaute.valeur() > Priorite::Haute.valeur());
        assert!(Priorite::Haute.valeur() > Priorite::Normale.valeur());
        assert_eq!(Priorite::TempsReel(99).valeur(), 99);
    }
}
