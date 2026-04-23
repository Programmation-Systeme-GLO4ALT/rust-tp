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
    TempsRéel(u8),  // niveau 0-99
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

impl GestionnaireProcessus {
    fn nouveau() -> Self { 
         GestionnaireProcessus {
            processus: Vec::new(),
            prochain_pid: 1,
        }
    }

    fn créer_processus(
        &mut self,
        nom: String,
        priorité: Priorité,
        mémoire_ko: u64,
        pid_parent: Option<u32>,
    ) -> u32 { 
        let pid = self.prochain_pid;
        self.prochain_pid += 1;
        
        let processus = Processus {
            pid,
            nom,
            état: EtatProcessus::Prêt,
            priorité,
            mémoire_ko,
            pid_parent,
        };
        
        self.processus.push(processus);
        pid
    }  // Retourne le PID

    fn trouver(&self, pid: u32) -> Option<&Processus> { 
        self.processus.iter().find(|p| p.pid == pid)
    }

    fn changer_état(
        &mut self,
        pid: u32,
        nouvel_état: EtatProcessus,
    ) -> Result<(), String> { 
        for p in &mut self.processus {
            if p.pid == pid {
                p.état = nouvel_état;
                return Ok(());
            }
        }
        Err(format!("PID {} non trouvé", pid))
    }

    fn mémoire_totale_utilisée(&self) -> u64 { 
        let mut total = 0;
        for p in &self.processus {
            total += p.mémoire_ko;
        }
        total
    }

    fn processus_par_état(&self, état: &EtatProcessus) -> Vec<&Processus> { 
        let mut result = Vec::new();
        for p in &self.processus {
            if &p.état == état {
                result.push(p);
            }
        }
        result
    }

    fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {
        // Change l'état en Terminé { code_retour: 0 }
        // Retourne le code de retour ou une erreur si PID introuvable
        for p in &mut self.processus {
            if p.pid == pid {
                p.état = EtatProcessus::Terminé { code_retour: 0 };
                return Ok(0);
            }
        }
        Err(format!("PID {} non trouvé", pid))
    }

    fn afficher_résumé(&self) { 
        println!("=== Gestionnaire de Processus ===");
        println!("Total processus: {}", self.processus.len());
        
        let mut total_mémoire = 0;
        for p in &self.processus {
            total_mémoire += p.mémoire_ko;
        }
        println!("Mémoire totale utilisée: {} Ko", total_mémoire);
        println!();
        
        for p in &self.processus {
            println!("PID: {}", p.pid);
            println!("  Nom: {}", p.nom);
            
            // Afficher l'état
            match &p.état {
                EtatProcessus::Prêt => println!("  État: Prêt"),
                EtatProcessus::EnExécution { cpu_id } => {
                    println!("  État: En exécution (CPU {})", cpu_id)
                }
                EtatProcessus::Bloqué { raison } => println!("  État: Bloqué ({})", raison),
                EtatProcessus::Terminé { code_retour } => {
                    println!("  État: Terminé (code {})", code_retour)
                }
                EtatProcessus::Zombie => println!("  État: Zombie"),
            }
            
            // Afficher la priorité
            match &p.priorité {
                Priorité::TrèsFaible => println!("  Priorité: Très faible"),
                Priorité::Faible => println!("  Priorité: Faible"),
                Priorité::Normale => println!("  Priorité: Normale"),
                Priorité::Haute => println!("  Priorité: Haute"),
                Priorité::TrèsHaute => println!("  Priorité: Très haute"),
                Priorité::TempsRéel(niveau) => println!("  Priorité: Temps réel ({})", niveau),
            }
            
            println!("  Mémoire: {} Ko", p.mémoire_ko);
            
            match p.pid_parent {
                Some(parent) => println!("  Parent PID: {}", parent),
                None => println!("  Parent: Aucun"),
            }
            println!();
        }
        
        // Afficher les statistiques par état
        let prêts = self.processus_par_état(&EtatProcessus::Prêt).len();
        let en_cours = self.processus_par_état(&EtatProcessus::EnExécution { cpu_id: 0 }).len();
        let bloqués = self.processus_par_état(&EtatProcessus::Bloqué { raison: String::new() }).len();
        let terminés = self.processus_par_état(&EtatProcessus::Terminé { code_retour: 0 }).len();
        let zombies = self.processus_par_état(&EtatProcessus::Zombie).len();
        
        println!("Statistiques par état:");
        if prêts > 0 { println!("  Prêt: {}", prêts); }
        if en_cours > 0 { println!("  En exécution: {}", en_cours); }
        if bloqués > 0 { println!("  Bloqué: {}", bloqués); }
        if terminés > 0 { println!("  Terminé: {}", terminés); }
        if zombies > 0 { println!("  Zombie: {}", zombies); }
    }
}

fn main() {
    let mut gp = GestionnaireProcessus::nouveau();

    // Créer init (PID 1)
    let init = gp.créer_processus(
        String::from("init"),
        Priorité::Haute,
        1024,
        None,  // pas de parent
    );

    // Créer des processus fils
    let bash = gp.créer_processus(
        String::from("bash"),
        Priorité::Normale,
        4096,
        Some(init),
    );

    // Changer les états
    gp.changer_état(bash, EtatProcessus::EnExécution { cpu_id: 0 }).unwrap();

    // Afficher le résumé
    gp.afficher_résumé();

    // Tuer un processus
    match gp.tuer_processus(bash) {
        Ok(code) => println!("bash terminé avec code {}", code),
        Err(e) => eprintln!("Erreur : {}", e),
    }
}
