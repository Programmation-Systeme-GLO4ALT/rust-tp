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
    TempsReel(u8),
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
        Self {
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

        match self.processus.iter_mut().find(|p| p.pid == pid) {
            Some(p) => {
                p.etat = nouvel_etat;
                Ok(())
            }
            None => Err(format!("Processus {} introuvable", pid)),
        }
    }

    fn memoire_totale_utilisee(&self) -> u64 {
        self.processus.iter().map(|p| p.memoire_ko).sum()
    }

    fn processus_par_etat(&self, etat: &EtatProcessus) -> Vec<&Processus> {

        self.processus
            .iter()
            .filter(|p| std::mem::discriminant(&p.etat) == std::mem::discriminant(etat))
            .collect()
    }

    fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {

        match self.processus.iter_mut().find(|p| p.pid == pid) {

            Some(p) => {
                p.etat = EtatProcessus::Termine { code_retour: 0 };
                Ok(0)
            }

            None => Err(format!("PID {} introuvable", pid)),
        }
    }

    fn afficher_resume(&self) {

        println!("===== Resume des processus =====");

        for p in &self.processus {

            println!(
                "PID: {} | Nom: {} | Memoire: {} ko | Etat: {:?}",
                p.pid,
                p.nom,
                p.memoire_ko,
                p.etat
            );
        }

        println!("-------------------------------");
        println!(
            "Memoire totale utilisee: {} ko",
            self.memoire_totale_utilisee()
        );
    }
}

fn main() {

    let mut gp = GestionnaireProcessus::nouveau();

    // creer init
    let init = gp.creer_processus(
        String::from("init"),
        Priorite::Haute,
        1024,
        None,
    );

    // creer bash
    let bash = gp.creer_processus(
        String::from("bash"),
        Priorite::Normale,
        4096,
        Some(init),
    );

    // changer etat
    gp.changer_etat(
        bash,
        EtatProcessus::EnExecution { cpu_id: 0 },
    ).unwrap();

    // afficher resume
    gp.afficher_resume();

    // tuer processus
    match gp.tuer_processus(bash) {

        Ok(code) => println!("bash termine avec code {}", code),

        Err(e) => eprintln!("Erreur : {}", e),
    }
}