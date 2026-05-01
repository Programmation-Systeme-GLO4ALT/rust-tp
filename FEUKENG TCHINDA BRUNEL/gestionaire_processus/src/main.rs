use std::io::{self, Write};
use std::collections::HashMap;

// États possibles d'un processus
#[derive(Debug, Clone, PartialEq)]
enum EtatProcessus {
    Pret,           // Prêt à être exécuté
    EnExecution,    // En cours d'exécution
    EnAttente,      // En attente d'une ressource
    Zombie,         // Terminé mais pas encore récupéré
}

// Priorité du processus
#[derive(Debug, Clone, PartialEq)]
enum Priorite {
    Haute,
    Normale,
    Basse,
}

impl Priorite {
    fn valeur(&self) -> u32 {
        match self {
            Priorite::Haute => 1,
            Priorite::Normale => 2,
            Priorite::Basse => 3,
        }
    }
}

// Structure d'un processus
#[derive(Debug, Clone)]
struct Processus {
    pid: u32,
    nom: String,
    etat: EtatProcessus,
    priorite: Priorite,
    memoire_utilisee: usize, // en MB
    temps_execution: u32,     // en secondes
}

impl Processus {
    // Créer un nouveau processus
    fn new(pid: u32, nom: String, priorite: Priorite, memoire: usize) -> Processus {
        Processus {
            pid,
            nom,
            etat: EtatProcessus::Pret,
            priorite,
            memoire_utilisee: memoire,
            temps_execution: 0,
        }
    }

    // Changer l'état du processus
    fn changer_etat(&mut self, nouvel_etat: EtatProcessus) {
        self.etat = nouvel_etat;
    }

    // Exécuter le processus (simulation)
    fn executer(&mut self, duree: u32) {
        if self.etat == EtatProcessus::Pret || self.etat == EtatProcessus::EnExecution {
            self.etat = EtatProcessus::EnExecution;
            self.temps_execution += duree;
        }
    }

    // Terminer le processus
    fn terminer(&mut self) {
        self.etat = EtatProcessus::Zombie;
    }

    // Afficher les infos du processus
    fn afficher(&self) {
        let etat_str = match self.etat {
            EtatProcessus::Pret => "🟢 Prêt",
            EtatProcessus::EnExecution => "🔵 En exécution",
            EtatProcessus::EnAttente => "🟡 En attente",
            EtatProcessus::Zombie => "⚫ Zombie",
        };

        let priorite_str = match self.priorite {
            Priorite::Haute => "🔴 Haute",
            Priorite::Normale => "🟠 Normale",
            Priorite::Basse => "🟢 Basse",
        };

        println!("  PID: {} | {} | {}", self.pid, self.nom, etat_str);
        println!("      Priorité: {} | Mémoire: {} MB | Temps: {}s",
            priorite_str, self.memoire_utilisee, self.temps_execution);
    }
}

// Gestionnaire de processus
struct GestionnaireProcessus {
    processus: HashMap<u32, Processus>,
    prochain_pid: u32,
    memoire_totale: usize,
    memoire_utilisee: usize,
}

impl GestionnaireProcessus {
    fn new(memoire_totale: usize) -> GestionnaireProcessus {
        GestionnaireProcessus {
            processus: HashMap::new(),
            prochain_pid: 1,
            memoire_totale,
            memoire_utilisee: 0,
        }
    }

    // Créer un nouveau processus
    fn creer_processus(&mut self, nom: String, priorite: Priorite, memoire: usize) -> Result<u32, String> {
        // Vérifier la mémoire disponible
        if self.memoire_utilisee + memoire > self.memoire_totale {
            return Err(format!("Mémoire insuffisante ! Besoin: {} MB, Disponible: {} MB",
                memoire, self.memoire_totale - self.memoire_utilisee));
        }

        let pid = self.prochain_pid;
        self.prochain_pid += 1;

        let processus = Processus::new(pid, nom, priorite, memoire);
        self.memoire_utilisee += memoire;
        self.processus.insert(pid, processus);

        Ok(pid)
    }

    // Trouver un processus
    fn trouver_processus(&self, pid: u32) -> Option<&Processus> {
        self.processus.get(&pid)
    }

    // Trouver un processus mutable
    fn trouver_processus_mut(&mut self, pid: u32) -> Option<&mut Processus> {
        self.processus.get_mut(&pid)
    }

    // Changer l'état d'un processus
    fn changer_etat(&mut self, pid: u32, nouvel_etat: EtatProcessus) -> Result<(), String> {
        match self.trouver_processus_mut(pid) {
            Some(proc) => {
                proc.changer_etat(nouvel_etat);
                Ok(())
            }
            None => Err(format!("Processus {} non trouvé", pid))
        }
    }

    // Exécuter un processus
    fn executer_processus(&mut self, pid: u32, duree: u32) -> Result<(), String> {
        match self.trouver_processus_mut(pid) {
            Some(proc) => {
                proc.executer(duree);
                Ok(())
            }
            None => Err(format!("Processus {} non trouvé", pid))
        }
    }

    // Terminer (tuer) un processus
    fn tuer_processus(&mut self, pid: u32) -> Result<(), String> {
        match self.processus.remove(&pid) {
            Some(proc) => {
                self.memoire_utilisee -= proc.memoire_utilisee;
                println!("🗑️ Processus {} '{}' terminé et libéré", proc.pid, proc.nom);
                Ok(())
            }
            None => Err(format!("Processus {} non trouvé", pid))
        }
    }

    // Lister tous les processus
    fn lister_processus(&self) {
        if self.processus.is_empty() {
            println!(" Aucun processus en cours");
            return;
        }

        println!("\n=== LISTE DES PROCESSUS ===");
        println!("Mémoire: {}/{} MB utilisée\n", self.memoire_utilisee, self.memoire_totale);

        let mut processus_list: Vec<&Processus> = self.processus.values().collect();
        processus_list.sort_by_key(|p| p.priorite.valeur());

        for proc in processus_list {
            proc.afficher();
            println!();
        }
        println!("===========================\n");
    }

    // Statistiques des processus
    fn afficher_statistiques(&self) {
        let total = self.processus.len();
        let pret = self.processus.values().filter(|p| p.etat == EtatProcessus::Pret).count();
        let execution = self.processus.values().filter(|p| p.etat == EtatProcessus::EnExecution).count();
        let attente = self.processus.values().filter(|p| p.etat == EtatProcessus::EnAttente).count();
        let zombie = self.processus.values().filter(|p| p.etat == EtatProcessus::Zombie).count();

        println!("\n=== STATISTIQUES ===");
        println!("Total: {}", total);
        println!("Prêts: {}", pret);
        println!("En exécution: {}", execution);
        println!("En attente: {}", attente);
        println!("Zombies: {}", zombie);
        println!("Mémoire: {}/{} MB", self.memoire_utilisee, self.memoire_totale);
        println!("===================\n");
    }
}

fn main() {
    let mut gestionnaire = GestionnaireProcessus::new(4096); // 4GB de mémoire totale

    println!("\n=== GESTIONNAIRE DE PROCESSUS (Unix-like) ===\n");

    // Créer quelques processus de démonstration
    let _ = gestionnaire.creer_processus("systemd".to_string(), Priorite::Haute, 512);
    let _ = gestionnaire.creer_processus("nginx".to_string(), Priorite::Normale, 256);
    let _ = gestionnaire.creer_processus("mysql".to_string(), Priorite::Normale, 1024);

    loop {
        println!("=== MENU PRINCIPAL ===");
        println!("1. Lister les processus");
        println!("2. Créer un processus");
        println!("3. Changer l'état d'un processus");
        println!("4. Exécuter un processus");
        println!("5. Terminer (tuer) un processus");
        println!("6. Afficher les statistiques");
        println!("7. Quitter");
        print!("\nChoix: ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();
        let choix = choix.trim();

        match choix {
            "1" => {
                gestionnaire.lister_processus();
            }
            "2" => {
                print!("Nom du processus: ");
                io::stdout().flush().unwrap();
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).unwrap();
                let nom = nom.trim().to_string();

                println!("Priorité:");
                println!("  1. Haute");
                println!("  2. Normale");
                println!("  3. Basse");
                print!("Choix (1-3): ");
                io::stdout().flush().unwrap();
                let mut priorite_choix = String::new();
                io::stdin().read_line(&mut priorite_choix).unwrap();

                let priorite = match priorite_choix.trim() {
                    "1" => Priorite::Haute,
                    "2" => Priorite::Normale,
                    "3" => Priorite::Basse,
                    _ => Priorite::Normale,
                };

                print!("Mémoire à allouer (MB): ");
                io::stdout().flush().unwrap();
                let mut memoire_str = String::new();
                io::stdin().read_line(&mut memoire_str).unwrap();
                let memoire: usize = memoire_str.trim().parse().unwrap_or(128);

                match gestionnaire.creer_processus(nom, priorite, memoire) {
                    Ok(pid) => println!(" Processus créé avec PID: {}", pid),
                    Err(e) => println!(" Erreur: {}", e),
                }
            }
            "3" => {
                print!("PID du processus: ");
                io::stdout().flush().unwrap();
                let mut pid_str = String::new();
                io::stdin().read_line(&mut pid_str).unwrap();
                let pid: u32 = pid_str.trim().parse().unwrap_or(0);

                println!("Nouvel état:");
                println!("  1. Prêt");
                println!("  2. En exécution");
                println!("  3. En attente");
                println!("  4. Zombie");
                print!("Choix (1-4): ");
                io::stdout().flush().unwrap();
                let mut etat_choix = String::new();
                io::stdin().read_line(&mut etat_choix).unwrap();

                let etat = match etat_choix.trim() {
                    "1" => EtatProcessus::Pret,
                    "2" => EtatProcessus::EnExecution,
                    "3" => EtatProcessus::EnAttente,
                    "4" => EtatProcessus::Zombie,
                    _ => EtatProcessus::Pret,
                };

                match gestionnaire.changer_etat(pid, etat) {
                    Ok(()) => println!(" État du processus {} modifié", pid),
                    Err(e) => println!(" {}", e),
                }
            }
            "4" => {
                print!("PID du processus: ");
                io::stdout().flush().unwrap();
                let mut pid_str = String::new();
                io::stdin().read_line(&mut pid_str).unwrap();
                let pid: u32 = pid_str.trim().parse().unwrap_or(0);

                print!("Durée d'exécution (secondes): ");
                io::stdout().flush().unwrap();
                let mut duree_str = String::new();
                io::stdin().read_line(&mut duree_str).unwrap();
                let duree: u32 = duree_str.trim().parse().unwrap_or(1);

                match gestionnaire.executer_processus(pid, duree) {
                    Ok(()) => println!(" Processus {} exécuté pendant {}s", pid, duree),
                    Err(e) => println!(" {}", e),
                }
            }
            "5" => {
                print!("PID du processus à terminer: ");
                io::stdout().flush().unwrap();
                let mut pid_str = String::new();
                io::stdin().read_line(&mut pid_str).unwrap();
                let pid: u32 = pid_str.trim().parse().unwrap_or(0);

                match gestionnaire.tuer_processus(pid) {
                    Ok(()) => println!(" Processus {} terminé", pid),
                    Err(e) => println!("{}", e),
                }
            }
            "6" => {
                gestionnaire.afficher_statistiques();
            }
            "7" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide !"),
        }
        println!();
    }
}