// Modélisation de Processus OS en Rust — TP4

#[derive(Debug, Clone)]
enum EtatProcessus {
    Pret,
    EnExecution { cpu_id: u8 },
    Bloque { raison: String },
    Termine { code_retour: i32 },
    Zombie,
}

#[derive(Debug, Clone)]
enum Priorite {
    TresFaible,
    Faible,
    Normale,
    Haute,
    TresHaute,
    TempsReel(u8), // niveau 0-99
}

#[derive(Debug)]
struct Processus {
    pid: u32,
    nom: String,
    etat: EtatProcessus,
    priorite: Priorite,
    memoire_ko: u64,
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

    fn creer_processus(
        &mut self,
        nom: String,
        priorite: Priorite,
        memoire_ko: u64,
        pid_parent: Option<u32>,
    ) -> u32 {
        let pid = self.prochain_pid;
        self.prochain_pid += 1;
        let processus = Processus {
            pid,
            nom,
            etat: EtatProcessus::Pret,
            priorite,
            memoire_ko,
            pid_parent,
        };
        self.processus.push(processus);
        pid
    }

    fn trouver(&self, pid: u32) -> Option<&Processus> {
        self.processus.iter().find(|p| p.pid == pid)
    }

    fn changer_etat(
        &mut self,
        pid: u32,
        nouvel_etat: EtatProcessus,
    ) -> Result<(), String> {
        if let Some(processus) = self.processus.iter_mut().find(|p| p.pid == pid) {
            processus.etat = nouvel_etat;
            Ok(())
        } else {
            Err(format!("Processus avec PID {} introuvable", pid))
        }
    }

    fn memoire_totale_utilisee(&self) -> u64 {
        self.processus.iter().map(|p| p.memoire_ko).sum()
    }

    fn processus_par_etat(&self, etat: &EtatProcessus) -> Vec<&Processus> {
        self.processus.iter().filter(|p| match (&p.etat, etat) {
            (EtatProcessus::Pret, EtatProcessus::Pret) => true,
            (EtatProcessus::EnExecution { .. }, EtatProcessus::EnExecution { .. }) => true,
            (EtatProcessus::Bloque { .. }, EtatProcessus::Bloque { .. }) => true,
            (EtatProcessus::Termine { .. }, EtatProcessus::Termine { .. }) => true,
            (EtatProcessus::Zombie, EtatProcessus::Zombie) => true,
            _ => false,
        }).collect()
    }

    fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {
        if let Some(processus) = self.processus.iter_mut().find(|p| p.pid == pid) {
            processus.etat = EtatProcessus::Termine { code_retour: 0 };
            Ok(0)
        } else {
            Err(format!("Processus avec PID {} introuvable", pid))
        }
    }

    fn afficher_resume(&self) {
        println!("=== Résumé des processus ===");
        for processus in &self.processus {
            println!(
                "PID: {}, Nom: {}, État: {:?}, Priorité: {:?}, Mémoire: {} Ko",
                processus.pid, processus.nom, processus.etat, processus.priorite, processus.memoire_ko
            );
        }
        println!("Mémoire totale utilisée: {} Ko", self.memoire_totale_utilisee());
    }
}

fn main() {
    let mut gp = GestionnaireProcessus::nouveau();

    // Créer le processus "init" (PID 1)
    let init = gp.creer_processus(
        String::from("init"),
        Priorite::Haute,
        1024,
        None,
    );

    // Créer un processus fils "bash"
    let bash = gp.creer_processus(
        String::from("bash"),
        Priorite::Normale,
        4096,
        Some(init),
    );

    // Changer l'état de "bash" en "EnExecution"
    gp.changer_etat(bash, EtatProcessus::EnExecution { cpu_id: 0 }).unwrap();

    // Afficher le résumé des processus
    gp.afficher_resume();

    // Tuer le processus "bash"
    match gp.tuer_processus(bash) {
        Ok(code) => println!("Processus {} terminé avec code {}", bash, code),
        Err(e) => eprintln!("Erreur : {}", e),
    }

    // Afficher à nouveau le résumé
    gp.afficher_resume();
}
