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
    Temps_Réel(u8),
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
    // Création du gestionnaire
    fn nouveau() -> Self {
        Self {
            processus: Vec::new(),
            prochain_pid: 1,
        }
    }

    // Créer un processus
    fn créer_processus(
        &mut self,
        nom: String,
        priorité: Priorité,
        mémoire_ko: u64,
        pid_parent: Option<u32>,
    ) -> u32 {
        let pid = self.prochain_pid;

        let p = Processus {
            pid,
            nom,
            état: EtatProcessus::Prêt,
            priorité,
            mémoire_ko,
            pid_parent,
        };

        self.processus.push(p);
        self.prochain_pid += 1;

        pid
    }

    // Trouver un processus
    fn trouver(&self, pid: u32) -> Option<&Processus> {
        self.processus.iter().find(|p| p.pid == pid)
    }

    // Changer l'état
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

    // Mémoire totale
    fn mémoire_totale_utilisée(&self) -> u64 {
        self.processus.iter().map(|p| p.mémoire_ko).sum()
    }

    // Filtrer par état
    fn processus_par_état(&self, état: &EtatProcessus) -> Vec<&Processus> {
        self.processus
            .iter()
            .filter(|p| std::mem::discriminant(&p.état) == std::mem::discriminant(état))
            .collect()
    }

    // Tuer un processus
    fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {
        match self.processus.iter_mut().find(|p| p.pid == pid) {
            Some(p) => {
                p.état = EtatProcessus::Terminé { code_retour: 0 };
                Ok(0)
            }
            None => Err(format!("PID {} introuvable", pid)),
        }
    }

    // Afficher résumé
    fn afficher_résumé(&self) {
        println!("***** RÉSUMÉ DES PROCESSUS *****");

        for p in &self.processus {
            println!(
                "PID: {} | Nom: {} | Mémoire: {} Ko",
                p.pid, p.nom, p.mémoire_ko
            );

            match &p.état {
                EtatProcessus::Prêt => println!("  État: Prêt"),
                EtatProcessus::EnExécution { cpu_id } => {
                    println!("  État: En exécution sur CPU {}", cpu_id)
                }
                EtatProcessus::Bloqué { raison } => {
                    println!("  État: Bloqué ({})", raison)
                }
                EtatProcessus::Terminé { code_retour } => {
                    println!("  État: Terminé (code {})", code_retour)
                }
                EtatProcessus::Zombie => println!("  État: Zombie"),
            }
        }

        println!(
            "\nMémoire totale utilisée: {} Ko",
            self.mémoire_totale_utilisée()
        );
    }
}

fn main() {
    let mut gp = GestionnaireProcessus::nouveau();

    let init = gp.créer_processus(
        String::from("init"),
        Priorité::Haute,
        1024,
        None,
    );

    let bash = gp.créer_processus(
        String::from("bash"),
        Priorité::Normale,
        4096,
        Some(init),
    );

    gp.changer_état(bash, EtatProcessus::EnExécution { cpu_id: 0 })
        .unwrap();

    gp.afficher_résumé();

    match gp.tuer_processus(bash) {
        Ok(code) => println!("bash terminé avec code {}", code),
        Err(e) => eprintln!("Erreur : {}", e),
    }
}