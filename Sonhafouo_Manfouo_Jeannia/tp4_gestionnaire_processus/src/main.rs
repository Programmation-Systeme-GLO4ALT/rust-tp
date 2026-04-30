#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum EtatProcessus {
    Prêt,
    EnExécution { cpu_id: u8 },
    Bloqué { raison: String },
    Terminé { code_retour: i32 },
    Zombie,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Priorité {
    TrèsFaible,
    Faible,
    Normale,
    Haute,
    TrèsHaute,
    TempsRéel(u8), // niveau 0-99
}

#[allow(dead_code)]
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

    fn trouver(&self, pid: u32) -> Option<&Processus> {
        self.processus.iter().find(|p| p.pid == pid)
    }

    fn changer_état(&mut self, pid: u32, nouvel_état: EtatProcessus) -> Result<(), String> {
        match self.processus.iter_mut().find(|p| p.pid == pid) {
            Some(p) => {
                p.état = nouvel_état;
                Ok(())
            }
            None => Err(format!("PID {} introuvable", pid)),
        }
    }

    fn mémoire_totale_utilisée(&self) -> u64 {
        self.processus
            .iter()
            .filter(|p| !matches!(p.état, EtatProcessus::Terminé { .. } | EtatProcessus::Zombie))
            .map(|p| p.mémoire_ko)
            .sum()
    }

    fn processus_par_état(&self, état: &EtatProcessus) -> Vec<&Processus> {
        self.processus
            .iter()
            .filter(|p| std::mem::discriminant(&p.état) == std::mem::discriminant(état))
            .collect()
    }

    fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {
        match self.processus.iter_mut().find(|p| p.pid == pid) {
            Some(p) => {
                p.état = EtatProcessus::Terminé { code_retour: 0 };
                Ok(0)
            }
            None => Err(format!("PID {} introuvable", pid)),
        }
    }

    fn afficher_résumé(&self) {
        println!("\n╔══════════════════════════════════════════════════╗");
        println!("║          GESTIONNAIRE DE PROCESSUS               ║");
        println!("╠══════════════════════════════════════════════════╣");
        println!("║ {:<6} {:<16} {:<18} {:<8}║", "PID", "NOM", "ÉTAT", "MEM(ko)");
        println!("╠══════════════════════════════════════════════════╣");
        for p in &self.processus {
            let état_str = match &p.état {
                EtatProcessus::Prêt => String::from("Prêt"),
                EtatProcessus::EnExécution { cpu_id } => format!("CPU#{}", cpu_id),
                EtatProcessus::Bloqué { raison } => format!("Bloqué({})", raison),
                EtatProcessus::Terminé { code_retour } => format!("Terminé({})", code_retour),
                EtatProcessus::Zombie => String::from("Zombie"),
            };
            let parent = p
                .pid_parent
                .map(|id| format!("(parent:{})", id))
                .unwrap_or_default();
            println!(
                "║ {:<6} {:<16} {:<18} {:<8}║",
                p.pid,
                format!("{}{}", p.nom, parent),
                état_str,
                p.mémoire_ko
            );
        }
        println!("╠══════════════════════════════════════════════════╣");
        println!(
            "║ Mémoire totale utilisée : {:<23}║",
            format!("{} ko", self.mémoire_totale_utilisée())
        );
        println!("╚══════════════════════════════════════════════════╝\n");
    }
}

fn main() {
    let mut gp = GestionnaireProcessus::nouveau();

    // Créer init (PID 1)
    let init = gp.créer_processus(String::from("init"), Priorité::Haute, 1024, None);

    // Créer des processus fils
    let bash = gp.créer_processus(String::from("bash"), Priorité::Normale, 4096, Some(init));
    let vim = gp.créer_processus(String::from("vim"), Priorité::Normale, 8192, Some(bash));
    let _daemon = gp.créer_processus(
        String::from("daemon"),
        Priorité::TempsRéel(10),
        512,
        Some(init),
    );

    // Changer les états
    gp.changer_état(init, EtatProcessus::EnExécution { cpu_id: 0 }).unwrap();
    gp.changer_état(bash, EtatProcessus::EnExécution { cpu_id: 1 }).unwrap();
    gp.changer_état(
        vim,
        EtatProcessus::Bloqué {
            raison: String::from("I/O"),
        },
    )
    .unwrap();

    gp.afficher_résumé();

    // Recherche par PID
    match gp.trouver(bash) {
        Some(p) => println!("Trouvé : {:?}", p),
        None => println!("PID introuvable"),
    }

    // Processus en cours d'exécution
    let en_exec = gp.processus_par_état(&EtatProcessus::EnExécution { cpu_id: 0 });
    println!("\nProcessus en exécution : {}", en_exec.len());
    for p in &en_exec {
        println!("  - [{}] {}", p.pid, p.nom);
    }

    // Tuer bash
    println!();
    match gp.tuer_processus(bash) {
        Ok(code) => println!("bash (PID {}) terminé avec code {}", bash, code),
        Err(e) => eprintln!("Erreur : {}", e),
    }

    // PID inexistant
    match gp.tuer_processus(999) {
        Ok(_) => {}
        Err(e) => println!("Erreur attendue : {}", e),
    }

    gp.afficher_résumé();

    println!(
        "Mémoire totale utilisée : {} ko",
        gp.mémoire_totale_utilisée()
    );
}
