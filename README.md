# Rust Practical Works — Collection de TP

## Présentation

Ce dépôt regroupe plusieurs travaux pratiques réalisés en Rust, couvrant des concepts fondamentaux du langage allant de la manipulation de base jusqu'à des notions avancées comme l'abstraction et la gestion d'erreurs.

Chaque TP est organisé dans un dossier indépendant avec sa propre logique et ses objectifs pédagogiques.

## Structure du projet

.
├── tp1-calculatrice_cli
├── tp2-gestion_memoire
├── tp3-analyse_chaine
├── tp4-gestion_processus


## Description des TP

### TP1 — Calculatrice CLI

- Application en ligne de commande permettant d'effectuer des opérations arithmétiques de base.
- Concepts abordés : entrées utilisateur, parsing d'arguments, structures de contrôle.

### TP2 — Gestion mémoire

- Exploration des mécanismes de gestion mémoire en Rust.
- Concepts abordés : ownership, borrowing, lifetimes.

### TP3 — Analyse de chaîne

- Manipulation et analyse de chaînes de caractères.
- Concepts abordés : `String` vs `&str`, parcours et transformation, fonctions utilitaires.

### TP4 — Gestion des processus

- Interaction avec le système pour gérer des processus.
- Concepts abordés : exécution de commandes système, processus enfants, entrées/sorties système.

## Exécution des TP

Chaque TP est un projet Rust indépendant. Pour exécuter un TP :

```bash
cd tpX-nom_du_tp
cargo run
```

Exemple :

```bash
cd tp1-calculatrice_cli
cargo run
```

## Utilisation d'un Makefile

Un `Makefile` à la racine permet de simplifier l'exécution des TP.

```makefile
run-tp1:
	cd tp1-calculatrice_cli && cargo run

run-tp2:
	cd tp2-gestion_memoire && cargo run

run-tp3:
	cd tp3-analyse_chaine && cargo run

run-tp4:
	cd tp4-gestion_processus && cargo run

build-all:
	for d in tp*; do (cd $$d && cargo build); done

clean-all:
	for d in tp*; do (cd $$d && cargo clean); done
```

Commandes disponibles :

```bash
make run-tp1
make build-all
make clean-all
```

## Auteur
- Felix Kamdjo, Travaux réalisés dans le cadre d'un apprentissage du langage Rust et des concepts systèmes associés.
