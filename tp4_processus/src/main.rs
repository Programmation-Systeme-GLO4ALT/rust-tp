#[derive(Debug, Clone, PartialEq)]
enum EtatProcessus { Prêt, EnExécution { cpu_id: u8 }, Bloqué { raison: String }, Terminé { code_retour: i32 }, Zombie }

#[derive(Debug, Clone)]
enum Priorite { Haute, Normale }

#[derive(Debug)]
struct Processus { pid: u32, nom: String, etat: EtatProcessus, priorite: Priorite, memoire_ko: u64, pid_parent: Option<u32> }

struct GestionnaireProcessus { processus: Vec<Processus>, prochain_pid: u32 }

impl GestionnaireProcessus {
    fn nouveau() -> Self {
        GestionnaireProcessus { processus: Vec::new(), prochain_pid: 1 }
    }

    fn creer_processus(&mut self, nom: String, priorite: Priorite, memoire_ko: u64, pid_parent: Option<u32>) -> u32 {
        let pid = self.prochain_pid;
        self.prochain_pid += 1;
        self.processus.push(Processus { pid, nom, etat: EtatProcessus::Prêt, priorite, memoire_ko, pid_parent });
        pid
    }

    fn changer_etat(&mut self, pid: u32, nouvel_etat: EtatProcessus) -> Result<(), String> {
        if let Some(p) = self.processus.iter_mut().find(|p| p.pid == pid) {
            p.etat = nouvel_etat;
            Ok(())
        } else {
            Err(String::from("PID introuvable"))
        }
    }

    fn tuer_processus(&mut self, pid: u32) -> Result<i32, String> {
        if let Some(p) = self.processus.iter_mut().find(|p| p.pid == pid) {
            p.etat = EtatProcessus::Terminé { code_retour: 0 };
            Ok(0)
        } else {
            Err(String::from("PID introuvable"))
        }
    }

    fn afficher_resume(&self) {
        println!("--- Résumé des processus ---");
        for p in &self.processus { println!("[{}] {} - {:?}", p.pid, p.nom, p.etat); }
    }
}

fn main() {
    let mut gp = GestionnaireProcessus::nouveau();
    let init = gp.creer_processus(String::from("init"), Priorite::Haute, 1024, None);
    let bash = gp.creer_processus(String::from("bash"), Priorite::Normale, 4096, Some(init));
    
    gp.changer_etat(bash, EtatProcessus::EnExécution { cpu_id: 0 }).unwrap();
    gp.afficher_resume();
    
    match gp.tuer_processus(bash) {
        Ok(code) => println!("bash terminé avec code {}", code),
        Err(e) => eprintln!("Erreur : {}", e),
    }
}