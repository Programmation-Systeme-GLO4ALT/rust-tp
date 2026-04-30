#[derive(Debug, Clone, PartialEq)]
enum EtatProcessus {
    Pret,
    EnExecution { cpu_id: u8 },
    Bloque { raison: String },
    Termine { code_retour: i32 },
    Zombie,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Priorite {
    TresFaible,
    Faible,
    Normale,
    Haute,
    TresHaute,
    TempsReel(u8), // niveau 0-99
}

#[derive(Debug, Clone)]
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

    #[allow(dead_code)]
    fn trouver(&self, pid: u32) -> Option<&Processus> {
        self.processus.iter().find(|p| p.pid == pid)
    }

    fn trouver_mut(&mut self, pid: u32) -> Option<&mut Processus> {
        self.processus.iter_mut().find(|p| p.pid == pid)
    }

    fn changer_etat(&mut self, pid: u32, nouvel_etat: EtatProcessus) -> Result<(), String> {
        let processus = self
            .trouver_mut(pid)
            .ok_or_else(|| format!("Processus avec PID {} introuvable", pid))?;

        // Vérification supplémentaire : un processus termine ne peut pas changer d'etat
        if let EtatProcessus::Termine { .. } = processus.etat {
            return Err(format!("Le processus {} est déjà terminé", pid));
        }

        processus.etat = nouvel_etat;
        Ok(())
    }

    fn memoire_totale_utilisee(&self) -> u64 {
        self.processus
            .iter()
            .filter(|p| {
                // Ne compter que les processus actifs (non terminés)
                !matches!(p.etat, EtatProcessus::Termine { .. } | EtatProcessus::Zombie)
            })
            .map(|p| p.memoire_ko)
            .sum()
    }

    fn processus_par_etat(&self, etat: &EtatProcessus) -> Vec<&Processus> {
        self.processus
            .iter()
            .filter(|p| std::mem::discriminant(&p.etat) == std::mem::discriminant(etat))
            .collect()
    }

    fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {
        // Trouver l'index du processus à tuer
        let index = self
            .processus
            .iter()
            .position(|p| p.pid == pid)
            .ok_or_else(|| format!("Processus avec PID {} introuvable", pid))?;

        // Vérifier s'il est déjà terminé
        if let EtatProcessus::Termine { code_retour } = self.processus[index].etat {
            return Ok(code_retour);
        }

        // Marquer comme terminé
        self.processus[index].etat = EtatProcessus::Termine { code_retour: 0 };

        // Marquer les fils comme Zombie
        for p in self.processus.iter_mut() {
            if p.pid_parent == Some(pid) && !matches!(p.etat, EtatProcessus::Termine { .. }) {
                p.etat = EtatProcessus::Zombie;
            }
        }

        Ok(0)
    }

    fn afficher_resume(&self) {
        println!("{}", "=".repeat(60));
        println!("GESTIONNAIRE DE PROCESSUS");
        println!("{}", "=".repeat(60));
        println!("Total processus: {}", self.processus.len());
        println!("Memoire totale utilisée: {} Ko", self.memoire_totale_utilisee());
        println!("Prochain PID disponible: {}", self.prochain_pid);
        println!();

        // Statistiques par état
        let etats = [
            ("Pret", EtatProcessus::Pret),
            ("EnExecution", EtatProcessus::EnExecution { cpu_id: 0 }),
            ("Bloque", EtatProcessus::Bloque {
                raison: String::new(),
            }),
            ("Termine", EtatProcessus::Termine { code_retour: 0 }),
            ("Zombie", EtatProcessus::Zombie),
        ];

        for (nom, etat_exemple) in etats {
            let count = self.processus_par_etat(&etat_exemple).len();
            if count > 0 {
                println!("  {}: {}", nom, count);
            }
        }
        println!();

        // Affichage détaillé des processus
        println!("DETAIL DES PROCESSUS:");
        println!("{:-<60}", "");
        for p in &self.processus {
            println!("PID: {}", p.pid);
            println!("  Nom: {}", p.nom);
            println!("  Etat: {:?}", p.etat);
            println!("  Priorite: {:?}", p.priorite);
            println!("  Memoire: {} Ko", p.memoire_ko);
            if let Some(parent) = p.pid_parent {
                println!("  Parent: {}", parent);
            }
            println!("  ---");
        }
        println!("{}", "=".repeat(60));
    }
}

impl Default for GestionnaireProcessus {
    fn default() -> Self {
        Self::nouveau()
    }
}

// Affichage personnalisé pour EtatProcessus
impl std::fmt::Display for EtatProcessus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EtatProcessus::Pret => write!(f, "Pret"),
            EtatProcessus::EnExecution { cpu_id } => write!(f, "EnExecution (CPU {})", cpu_id),
            EtatProcessus::Bloque { raison } => write!(f, "Bloque ({})", raison),
            EtatProcessus::Termine { code_retour } => write!(f, "Termine (code {})", code_retour),
            EtatProcessus::Zombie => write!(f, "Zombie"),
        }
    }
}

fn main() {
    let mut gp = GestionnaireProcessus::nouveau();

    // Créer init (PID 1)
    let init = gp.creer_processus(String::from("init"), Priorite::Haute, 1024, None);

    // Créer des processus fils
    let bash = gp.creer_processus(
        String::from("bash"),
        Priorite::Normale,
        4096,
        Some(init),
    );

    let systemd = gp.creer_processus(
        String::from("systemd"),
        Priorite::TresHaute,
        2048,
        Some(init),
    );

    let sshd = gp.creer_processus(
        String::from("sshd"),
        Priorite::Haute,
        3072,
        Some(init),
    );

    // Changer les etats
    gp.changer_etat(bash, EtatProcessus::EnExecution { cpu_id: 0 })
        .unwrap();
    gp.changer_etat(systemd, EtatProcessus::EnExecution { cpu_id: 1 })
        .unwrap();
    gp.changer_etat(
        sshd,
        EtatProcessus::Bloque {
            raison: String::from("attente connexion"),
        },
    )
    .unwrap();

    // Afficher le resume
    gp.afficher_resume();

    // Tuer un processus
    match gp.tuer_processus(bash) {
        Ok(code) => println!("\n✅ bash terminé avec code {}", code),
        Err(e) => eprintln!("\n❌ Erreur : {}", e),
    }

    // Afficher le résumé après avoir tué bash
    println!("\nAprès avoir tué bash:");
    gp.afficher_resume();

    // Vérifier la memoire totale utilisée
    println!(
        "\nMemoire totale active: {} Ko",
        gp.memoire_totale_utilisee()
    );

    // Compter les processus par etat
    println!(
        "\nProcessus en etat 'Pret': {}",
        gp.processus_par_etat(&EtatProcessus::Pret).len()
    );
    println!(
        "Processus en etat 'EnExecution': {}",
        gp.processus_par_etat(&EtatProcessus::EnExecution { cpu_id: 0 })
            .len()
    );
    println!(
        "Processus en etat 'Bloque': {}",
        gp.processus_par_etat(&EtatProcessus::Bloque {
            raison: String::new()
        })
        .len()
    );
}