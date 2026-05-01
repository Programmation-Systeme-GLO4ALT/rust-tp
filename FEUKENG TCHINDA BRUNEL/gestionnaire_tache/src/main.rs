use std::io::{self, Write};

// Structure pour une tâche
#[derive(Debug, Clone)]
struct Tache {
    titre: String,
    description: String,
    terminee: bool,
}

impl Tache {
    // Créer une nouvelle tâche
    fn new(titre: String, description: String) -> Tache {
        Tache {
            titre,
            description,
            terminee: false,
        }
    }

    // Marquer comme terminée (prend ownership et retourne modifié)
    fn terminer(mut self) -> Tache {
        self.terminee = true;
        self
    }

    // Modifier la description
    fn modifier_description(mut self, nouvelle: String) -> Tache {
        self.description = nouvelle;
        self
    }
}

// Structure du gestionnaire
struct Gestionnaire {
    taches: Vec<Tache>,
}

impl Gestionnaire {
    fn new() -> Gestionnaire {
        Gestionnaire { taches: Vec::new() }
    }

    // Ajouter une tâche (prend ownership)
    fn ajouter_tache(&mut self, tache: Tache) {
        self.taches.push(tache);
    }

    // Supprimer une tâche (retourne la tâche supprimée)
    fn supprimer_tache(&mut self, index: usize) -> Option<Tache> {
        if index < self.taches.len() {
            Some(self.taches.remove(index))
        } else {
            None
        }
    }

    // Marquer une tâche comme terminée
    fn terminer_tache(&mut self, index: usize) -> bool {
        if index < self.taches.len() {
            // On prend ownership de la tâche, on la termine, et on la remet
            let tache = self.taches.remove(index);
            let tache_terminee = tache.terminer();
            self.taches.push(tache_terminee);
            true
        } else {
            false
        }
    }

    // Afficher toutes les tâches
    fn afficher(&self) {
        if self.taches.is_empty() {
            println!(" Aucune tâche !");
            return;
        }

        println!("\n=== LISTE DES TÂCHES ===");
        for (i, tache) in self.taches.iter().enumerate() {
            let statut = if tache.terminee { "✓" } else { "□" };
            println!("{}. [{}] {} - {}", i + 1, statut, tache.titre, tache.description);
        }
        println!("========================\n");
    }

    // Compter les tâches en cours
    fn compter_taches_en_cours(&self) -> usize {
        self.taches.iter().filter(|t| !t.terminee).count()
    }
}

fn main() {
    let mut gestionnaire = Gestionnaire::new();

    loop {
        println!("\n=== GESTIONNAIRE DE TÂCHES ===");
        println!("1. Ajouter une tâche");
        println!("2. Voir les tâches");
        println!("3. Terminer une tâche");
        println!("4. Supprimer une tâche");
        println!("5. Voir le nombre de tâches en cours");
        println!("6. Quitter");
        print!("Choix: ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();
        let choix = choix.trim();

        match choix {
            "1" => {
                // Ajouter une tâche
                print!("Titre: ");
                io::stdout().flush().unwrap();
                let mut titre = String::new();
                io::stdin().read_line(&mut titre).unwrap();

                print!("Description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();

                let tache = Tache::new(
                    titre.trim().to_string(),
                    description.trim().to_string(),
                );
                gestionnaire.ajouter_tache(tache);
                println!(" Tâche ajoutée !");
            }
            "2" => {
                gestionnaire.afficher();
            }
            "3" => {
                gestionnaire.afficher();
                print!("Numéro de la tâche à terminer: ");
                io::stdout().flush().unwrap();
                let mut num = String::new();
                io::stdin().read_line(&mut num).unwrap();
                let num: usize = num.trim().parse().unwrap_or(0);

                if gestionnaire.terminer_tache(num - 1) {
                    println!(" Tâche terminée !");
                } else {
                    println!(" Numéro invalide !");
                }
            }
            "4" => {
                gestionnaire.afficher();
                print!("Numéro de la tâche à supprimer: ");
                io::stdout().flush().unwrap();
                let mut num = String::new();
                io::stdin().read_line(&mut num).unwrap();
                let num: usize = num.trim().parse().unwrap_or(0);

                if let Some(tache) = gestionnaire.supprimer_tache(num - 1) {
                    println!(" Tâche supprimée: {}", tache.titre);
                } else {
                    println!(" Numéro invalide !");
                }
            }
            "5" => {
                let en_cours = gestionnaire.compter_taches_en_cours();
                println!(" Tâches en cours: {}", en_cours);
            }
            "6" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide !"),
        }
    }
}