#[derive(Debug, Clone)]
enum EtatProcessus {
    Pret,
    EnExecution { cpu_id: u8 },
    Bloque { raison: String },
    Termine { code_retour: i32 },
}

#[derive(Debug, Clone)]
enum Priorite {
    Faible,
    Normale,
    Haute,
}

#[derive(Debug, Clone)]
struct Processus {
    pid: u32,
    nom: String,
    etat: EtatProcessus,
    priorite: Priorite,
}

struct Gestionnaire {
    processus: Vec<Processus>,
    prochain_pid: u32,
}

impl Gestionnaire {
    fn nouveau() -> Self {
        Self {
            processus: Vec::new(),
            prochain_pid: 1,
        }
    }

    fn creer(&mut self, nom: String, priorite: Priorite) -> u32 {
        let pid = self.prochain_pid;
        self.prochain_pid += 1;

        let p = Processus {
            pid,
            nom,
            etat: EtatProcessus::Pret,
            priorite,
        };

        self.processus.push(p);
        pid
    }

    fn afficher(&self) {
        for p in &self.processus {
            println!("{:?}", p);
        }
    }
}

fn main() {
    let mut g = Gestionnaire::nouveau();

    g.creer(String::from("init"), Priorite::Haute);
    g.creer(String::from("bash"), Priorite::Normale);

    g.afficher();
}
