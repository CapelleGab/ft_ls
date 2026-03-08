# ft_ls

> Re-implementation de la commande Unix `ls` en Rust

## A Propos

**ft_ls** est un clone de la commande `ls` ecrit en Rust. Il reproduit les comportements essentiels de `ls` : listing de fichiers, format long, fichiers caches, tri par date, ordre inverse et parcours recursif.

## Fonctionnalites

- **Listing basique** : Affiche les fichiers et dossiers d'un repertoire
- **Format long (`-l`)** : Permissions, liens, taille, date de modification
- **Fichiers caches (`-a`)** : Inclut les fichiers commencant par `.`
- **Tri par date (`-t`)** : Trie par date de modification (plus recent en premier)
- **Ordre inverse (`-r`)** : Inverse l'ordre de tri
- **Recursif (`-R`)** : Parcourt les sous-repertoires recursivement
- **Multi-chemins** : Accepte plusieurs repertoires en argument

## Architecture

```
ft_ls/
├── src/
│   ├── main.rs       # Point d'entree, parsing des arguments, orchestration
│   ├── entry.rs      # Structure FileEntry (representation d'un fichier)
│   ├── listing.rs    # Lecture de repertoire, filtrage et tri
│   └── display.rs    # Affichage format court et format long
├── Cargo.toml        # Configuration du projet et dependances
└── Makefile          # Commandes de build et execution
```

## Demarrage Rapide

### Prerequis

- Rust 1.85+ (edition 2024)

### Build

```bash
cargo build
```

### Utilisation

```bash
# Listing basique
./target/debug/ft_ls

# Format long
./target/debug/ft_ls -l /tmp

# Fichiers caches + tri par date inversee
./target/debug/ft_ls -a -t -r

# Recursif sur plusieurs chemins
./target/debug/ft_ls -R src tests

# Avec le Makefile
make run ARGS="-l -a /tmp"
```

### Options

| Option | Description |
|--------|-------------|
| `-l`, `--long-format` | Affichage detaille (permissions, taille, date) |
| `-a`, `--show-hidden` | Affiche les fichiers caches |
| `-r`, `--rev` | Inverse l'ordre de tri |
| `-t`, `--order-by-update-date` | Tri par date de modification |
| `-R`, `--recursive` | Listing recursif des sous-repertoires |

## Documentation

La documentation complete est disponible dans [`docs/`](./docs/README.md).

| Section | Description |
|---------|-------------|
| [Architecture](./docs/architecture.md) | Architecture globale du projet |
| [Core](./docs/core.md) | Modules principaux (entry, listing, display) |
| [Guides](./docs/guides/) | Contribution et developpement |

## Developpement

| Commande | Description |
|----------|-------------|
| `cargo build` | Compile le projet |
| `cargo run -- [ARGS]` | Execute avec des arguments |
| `make run ARGS="..."` | Build + execute |
| `make clean` | Nettoie les artefacts de build |

## Licence

Projet personnel / educatif.
