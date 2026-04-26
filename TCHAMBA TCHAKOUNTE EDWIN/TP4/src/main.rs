// =============================================================
// TP4 — Modélisation de Processus OS
// Tchamba Tchakounte Edwin
//
// Démonstration : structs, enums, pattern matching, Option, Result
// =============================================================

use std::mem::discriminant;

// État dans lequel peut se trouver un processus
#[derive(Debug, Clone)]
enum EtatProcessus {
    Pret,
    EnExecution { cpu_id: u8 },
    Bloque { raison: String },
    Termine { code_retour: i32 },
    Zombie,
}

// Niveau de priorité d'un processus
#[derive(Debug, Clone)]
enum Priorite {
    TresFaible,
    Faible,
    Normale,
    Haute,
    TresHaute,
    TempsReel(u8), // niveau 0-99
}

// Représentation d'un processus
#[derive(Debug)]
struct Processus {
    pid: u32,
    nom: String,
    etat: EtatProcessus,
    priorite: Priorite,
    memoire_ko: u64,
    pid_parent: Option<u32>, // None si pas de parent (init)
}

// Gestionnaire central : contient tous les processus + le prochain PID
#[derive(Debug)]
struct GestionnaireProcessus {
    processus: Vec<Processus>,
    prochain_pid: u32,
}

impl GestionnaireProcessus {
    // Crée un gestionnaire vide. Le 1er PID sera 1 (par convention Unix)
    fn nouveau() -> Self {
        GestionnaireProcessus {
            processus: Vec::new(),
            prochain_pid: 1,
        }
    }

    // Crée un nouveau processus, l'ajoute à la liste et retourne son PID
    fn creer_processus(
        &mut self,
        nom: String,
        priorite: Priorite,
        memoire_ko: u64,
        pid_parent: Option<u32>,
    ) -> u32 {
        let pid = self.prochain_pid;
        self.prochain_pid += 1;

        let p = Processus {
            pid,
            nom,
            etat: EtatProcessus::Pret,
            priorite,
            memoire_ko,
            pid_parent,
        };
        self.processus.push(p);
        pid
    }

    // Cherche un processus par PID. None s'il n'existe pas.
    fn trouver(&self, pid: u32) -> Option<&Processus> {
        self.processus.iter().find(|p| p.pid == pid)
    }

    // Change l'état d'un processus. Erreur si PID introuvable.
    fn changer_etat(
        &mut self,
        pid: u32,
        nouvel_etat: EtatProcessus,
    ) -> Result<(), String> {
        match self.processus.iter_mut().find(|p| p.pid == pid) {
            Some(p) => {
                p.etat = nouvel_etat;
                Ok(())
            }
            None => Err(format!("Processus PID {} introuvable", pid)),
        }
    }

    // Somme de la mémoire utilisée par tous les processus
    fn memoire_totale_utilisee(&self) -> u64 {
        self.processus.iter().map(|p| p.memoire_ko).sum()
    }

    // Retourne tous les processus dont l'état correspond à `etat`.
    // On compare juste la "variante" de l'enum, pas les champs internes.
    fn processus_par_etat(&self, etat: &EtatProcessus) -> Vec<&Processus> {
        self.processus
            .iter()
            .filter(|p| discriminant(&p.etat) == discriminant(etat))
            .collect()
    }

    // Termine un processus (état Termine code 0). Renvoie le code de retour
    // ou une erreur si PID introuvable.
    fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {
        let code = 0;
        self.changer_etat(pid, EtatProcessus::Termine { code_retour: code })?;
        Ok(code)
    }

    // Affiche un résumé lisible du système
    fn afficher_resume(&self) {
        println!("\n┌─────────────────────────────────────────────────────┐");
        println!("│         RÉSUMÉ DU GESTIONNAIRE DE PROCESSUS         │");
        println!("├─────────────────────────────────────────────────────┤");
        println!("│ Nombre de processus : {:<29} │", self.processus.len());
        println!(
            "│ Mémoire totale     : {} ko{:<24} │",
            self.memoire_totale_utilisee(),
            ""
        );
        println!("├─────────────────────────────────────────────────────┤");
        for p in &self.processus {
            // Formatage lisible de l'état
            let etat_str = match &p.etat {
                EtatProcessus::Pret => String::from("Prêt"),
                EtatProcessus::EnExecution { cpu_id } => {
                    format!("En exécution (CPU {})", cpu_id)
                }
                EtatProcessus::Bloque { raison } => format!("Bloqué ({})", raison),
                EtatProcessus::Termine { code_retour } => {
                    format!("Terminé (code {})", code_retour)
                }
                EtatProcessus::Zombie => String::from("Zombie"),
            };
            // Formatage de la priorité
            let prio_str = match &p.priorite {
                Priorite::TresFaible => String::from("TresFaible"),
                Priorite::Faible => String::from("Faible"),
                Priorite::Normale => String::from("Normale"),
                Priorite::Haute => String::from("Haute"),
                Priorite::TresHaute => String::from("TresHaute"),
                Priorite::TempsReel(n) => format!("TempsReel({})", n),
            };
            // Affichage du parent
            let parent_str = match p.pid_parent {
                Some(ppid) => format!("parent={}", ppid),
                None => String::from("(racine)"),
            };
            println!(
                "│ PID {:<3} | {:<10} | {:<22} | {:<8} │",
                p.pid, p.nom, etat_str, prio_str
            );
            println!("│         | mem={:<5} ko | {:<31} │", p.memoire_ko, parent_str);
        }
        println!("└─────────────────────────────────────────────────────┘");
    }
}

fn main() {
    let mut gp = GestionnaireProcessus::nouveau();

    // Création de processus type Unix
    let init = gp.creer_processus(String::from("init"), Priorite::Haute, 1024, None);
    let bash = gp.creer_processus(
        String::from("bash"),
        Priorite::Normale,
        4096,
        Some(init),
    );
    let firefox = gp.creer_processus(
        String::from("firefox"),
        Priorite::Normale,
        524288,
        Some(bash),
    );
    let cron = gp.creer_processus(
        String::from("cron"),
        Priorite::Faible,
        2048,
        Some(init),
    );
    let audio = gp.creer_processus(
        String::from("audio_rt"),
        Priorite::TempsReel(99),
        8192,
        Some(init),
    );

    // Changements d'état
    gp.changer_etat(bash, EtatProcessus::EnExecution { cpu_id: 0 })
        .unwrap();
    gp.changer_etat(firefox, EtatProcessus::EnExecution { cpu_id: 1 })
        .unwrap();
    gp.changer_etat(
        cron,
        EtatProcessus::Bloque {
            raison: String::from("attente_io"),
        },
    )
    .unwrap();

    // Affichage initial
    gp.afficher_resume();

    // Recherche d'un processus
    println!("\n>>> Recherche du PID {}", firefox);
    match gp.trouver(firefox) {
        Some(p) => println!("Trouvé : {} (mémoire {} ko)", p.nom, p.memoire_ko),
        None => println!("Introuvable"),
    }

    // Recherche d'un PID inexistant
    println!("\n>>> Recherche du PID 999 (inexistant)");
    match gp.trouver(999) {
        Some(p) => println!("Trouvé : {}", p.nom),
        None => println!("Aucun processus avec ce PID"),
    }

    // Liste des processus en exécution
    println!("\n>>> Processus actuellement en exécution :");
    let en_cours = gp.processus_par_etat(&EtatProcessus::EnExecution { cpu_id: 0 });
    for p in &en_cours {
        println!("  - {} (PID {})", p.nom, p.pid);
    }

    // Tuer firefox
    println!("\n>>> Tentative de tuer firefox (PID {})", firefox);
    match gp.tuer_processus(firefox) {
        Ok(code) => println!("firefox terminé avec code {}", code),
        Err(e) => eprintln!("Erreur : {}", e),
    }

    // Tuer un PID inexistant pour montrer la gestion d'erreur
    println!("\n>>> Tentative de tuer le PID 999 (inexistant)");
    match gp.tuer_processus(999) {
        Ok(code) => println!("Terminé avec code {}", code),
        Err(e) => eprintln!("Erreur : {}", e),
    }

    // Affichage final
    gp.afficher_resume();

    // Mémoire totale
    println!(
        "\n>>> Mémoire totale utilisée : {} ko",
        gp.memoire_totale_utilisee()
    );
    let _ = audio;
}
