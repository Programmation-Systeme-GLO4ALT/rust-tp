// PARTIE 1 : Définition des types

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
}

#[derive(Debug, Clone)]
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



// PARTIE 2 : Implémentation du gestionnaire

impl GestionnaireProcessus {

    // Créer un nouveau gestionnaire vide
    fn nouveau() -> Self {
        GestionnaireProcessus {
            processus: Vec::new(),
            prochain_pid: 1,
        }
    }

    // Créer un processus et l'ajouter à la liste
    fn créer_processus(
        &mut self,
        nom: String,
        priorité: Priorité,
        mémoire_ko: u64,
        pid_parent: Option<u32>,
    ) -> u32 {
        let pid = self.prochain_pid;
        let processus = Processus {
            pid,
            nom,
            état: EtatProcessus::Prêt,  
            priorité,
            mémoire_ko,
            pid_parent,
        };
        self.processus.push(processus);
        self.prochain_pid += 1;
        pid  
    }

    // Trouver un processus par son PID
    fn trouver(&self, pid: u32) -> Option<&Processus> {
        self.processus.iter().find(|p| p.pid == pid)
    }

    // Changer l'état d'un processus
    fn changer_état(
        &mut self,
        pid: u32,
        nouvel_état: EtatProcessus,
    ) -> Result<(), String> {
        match self.processus.iter_mut().find(|p| p.pid == pid) {
            Some(p) => {
                p.état = nouvel_état;
                Ok(())
            }
            None => Err(format!("PID {} introuvable", pid)),
        }
    }

    // Calculer la mémoire totale utilisée
    fn mémoire_totale_utilisée(&self) -> u64 {
        self.processus.iter().map(|p| p.mémoire_ko).sum()
    }

    // Tuer un processus (le passer à Terminé)
    fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {
        match self.processus.iter_mut().find(|p| p.pid == pid) {
            Some(p) => {
                p.état = EtatProcessus::Terminé { code_retour: 0 };
                Ok(0)
            }
            None => Err(format!("PID {} introuvable", pid)),
        }
    }

    // Afficher un résumé de tous les processus
    fn afficher_résumé(&self) {
        println!("\n{:-<55}", "");
        println!("{:<6} {:<12} {:<20} {:<8}", "PID", "NOM", "ÉTAT", "MEM(Ko)");
        println!("{:-<55}", "");

        for p in &self.processus {
            let état_str = match &p.état {
                EtatProcessus::Prêt => String::from("Prêt"),
                EtatProcessus::EnExécution { cpu_id } =>
                    format!("EnExéc(cpu{})", cpu_id),
                EtatProcessus::Bloqué { raison } =>
                    format!("Bloqué({})", raison),
                EtatProcessus::Terminé { code_retour } =>
                    format!("Terminé({})", code_retour),
                EtatProcessus::Zombie => String::from("Zombie"),
            };

            let parent_str = match p.pid_parent {
                Some(ppid) => format!("(parent:{})", ppid),
                None => String::from("(root)"),
            };

            println!(
                "{:<6} {:<12} {:<20} {:<8} {}",
                p.pid, p.nom, état_str, p.mémoire_ko, parent_str
            );
        }
        println!("{:-<55}", "");
        println!("Mémoire totale : {} Ko\n", self.mémoire_totale_utilisée());
    }
}



// PARTIE 3 : Programme principal

fn main() {
    println!("=== Gestionnaire de Processus ===\n");

    let mut gp = GestionnaireProcessus::nouveau();

    // Créer init (PID 1) — le processus racine
    let init = gp.créer_processus(
        String::from("init"),
        Priorité::Haute,
        1024,
        None,  
    );

    // Créer des processus fils
    let bash = gp.créer_processus(
        String::from("bash"),
        Priorité::Normale,
        4096,
        Some(init),
    );

    let vim = gp.créer_processus(
        String::from("vim"),
        Priorité::Faible,
        2048,
        Some(bash),
    );

    let cargo = gp.créer_processus(
        String::from("cargo"),
        Priorité::Haute,
        8192,
        Some(bash),
    );

    // Afficher l'état initial
    println!("--- État initial ---");
    gp.afficher_résumé();

    // Changer des états
    gp.changer_état(init, EtatProcessus::EnExécution { cpu_id: 0 }).unwrap();
    gp.changer_état(bash, EtatProcessus::EnExécution { cpu_id: 1 }).unwrap();
    gp.changer_état(vim,  EtatProcessus::Bloqué {
        raison: String::from("lecture disque")
    }).unwrap();

    println!("--- Après changements d'état ---");
    gp.afficher_résumé();

    // Tuer vim
    match gp.tuer_processus(vim) {
        Ok(code) => println!("✓ vim terminé avec code {}", code),
        Err(e)   => eprintln!("✗ Erreur : {}", e),
    }

    // Tenter de tuer un PID inexistant
    match gp.tuer_processus(99) {
        Ok(code) => println!("✓ PID 99 terminé avec code {}", code),
        Err(e)   => eprintln!("✗ Erreur : {}", e),
    }

    // Rechercher un processus
    match gp.trouver(cargo) {
        Some(p) => println!("\n✓ Trouvé : {} (PID {})", p.nom, p.pid),
        None    => println!("\n✗ Processus introuvable"),
    }

    // Afficher l'état final
    println!("\n--- État final ---");
    gp.afficher_résumé();
}