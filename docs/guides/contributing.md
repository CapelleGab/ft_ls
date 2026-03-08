# Contribuer a ft_ls

## Setup

```bash
git clone <url>
cd ft_ls
cargo build
```

## Workflow

1. Cree une branche : `git checkout -b feature/ma-feature`
2. Code et teste : `cargo run -- -l /tmp`
3. Verifie : `cargo clippy && cargo fmt --check`
4. Commit et push

## Structure du Code

Chaque fonctionnalite correspond a une etape du pipeline :

| Etape | Fichier | Quand modifier |
|-------|---------|----------------|
| Modele de donnees | `entry.rs` | Ajouter des champs/methodes a FileEntry |
| Lecture/Filtrage/Tri | `listing.rs` | Nouveau flag de tri ou filtre |
| Affichage | `display.rs` | Modifier le format de sortie |
| Arguments CLI | `main.rs` | Ajouter un nouveau flag |

## Conventions

- Edition Rust 2024
- `cargo fmt` pour le formatage
- `cargo clippy` sans warnings
- Messages de commit courts et descriptifs
