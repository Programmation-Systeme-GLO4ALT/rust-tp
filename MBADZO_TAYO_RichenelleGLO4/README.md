## Structure de mes tps
```
tp1/
├── calculatrice_cli/   
└── gestion_taches/   
tp2/
├── programme1/   # Move sur Vec
└── programme2/   # Ownership entre fonctions
tp3/
tp4/
```

## Lancer un TP
```bash
cd tp1/calculatrice_cli && cargo run -- 10 + 5 && cargo run 7 * 6
cd tp1/gestion_taches && cargo run 
cd tp2/programme1      && cargo run
cd tp2/programme2      && cargo run
cd tp3       && cargo run && cargo test
cd tp4    && cargo run
```

## Réponses TP2
**Programme 1** : `v` est *moved* dans `v2`, `v.len()` est interdit ensuite.
Correction: utiliser `v2.len()` ou `let v2 = v.clone()`.

**Programme 2** : `somme()` consomme `nombres`, impossible de l'afficher après.
Correction: `fn somme(v: Vec<i32>) -> (i32, Vec<i32>)` — retourner le vecteur.

## Concepts abordes lors de ces tps 
| TP | Concept clé |
|---|---|
| TP1 | `cargo`, `match`, `Result<T,E>`, arguments CLI |
| TP2 | Ownership, Move vs Copy, transfert entre fonctions |
| TP3 | Borrowing `&str`, lifetimes `'a`, slices `&[T]` |
| TP4 | Structs, Enums, pattern matching, `Option`, `Result` |
