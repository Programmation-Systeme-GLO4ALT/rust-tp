---

## TP 1 — Calculatrice CLI

```bash
cd calculatrice_cli
cargo build
```

**Exécution :**
```bash
cargo run -- 10 + 5       # → 10 + 5 = 15
cargo run -- 20 - 8       # → 20 - 8 = 12
cargo run -- 6 "*" 7      # → 6 * 7 = 42
cargo run -- 10 / 0       # → Erreur : Division par zéro impossible
cargo run                 # → mode interactif (tapez "quitter" pour sortir)
```

**Tests :**
```bash
cargo test
```

---

## TP 2 — Gestion Mémoire Manuelle

```bash
cd gestionnaire_memoire_manuelle
cargo build
```

**Exécution :**
```bash
cargo run
```

**Tests :**
```bash
cargo test
```

---

## TP 3 — Analyseur de Chaînes

```bash
cd analyseur_chaines
cargo build
```

**Exécution :**
```bash
cargo run
```

**Tests :**
```bash
cargo test
```

---

## TP 4 — Gestionnaire de Processus OS

```bash
cd modelisation_processus_os
cargo build
```

**Exécution :**
```bash
cargo run
```

**Tests :**
```bash
cargo test
```

---

## Commandes utiles

| Commande | Description |
|---|---|
| `cargo build` | Compile le projet |
| `cargo run` | Compile et exécute |
| `cargo run -- arg1 arg2` | Exécute avec arguments |
| `cargo test` | Lance tous les tests |
| `cargo clippy` | Analyse statique |
| `cargo fmt` | Formate le code |
