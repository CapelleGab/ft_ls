# Documentation ft_ls

Bienvenue dans la documentation complete de **ft_ls**.

## Table des Matieres

### Demarrage
| Guide | Description |
|-------|-------------|
| [Getting Started](./getting-started.md) | Guide de demarrage complet |

### Architecture
| Document | Description |
|----------|-------------|
| [Architecture](./architecture.md) | Vue d'ensemble de l'architecture et du flux de donnees |

### Modules Core
| Module | Fichier | Description | Docs |
|--------|---------|-------------|------|
| **Entry** | `src/entry.rs` | Structure de donnees representant un fichier | [Details](./core/entry.md) |
| **Listing** | `src/listing.rs` | Lecture de repertoire, filtrage et tri | [Details](./core/listing.md) |
| **Display** | `src/display.rs` | Formatage et affichage des resultats | [Details](./core/display.md) |
| **Main** | `src/main.rs` | Point d'entree et orchestration | [Details](./core/main.md) |

### Guides
| Guide | Description |
|-------|-------------|
| [Contributing](./guides/contributing.md) | Comment contribuer au projet |

## Carte du Projet

```
Arguments (clap)
      │
      v
  main::run()
      │
      ├──> listing::list_entries()  ──> Vec<FileEntry>
      │         │
      │         └──> entry::FileEntry::new()
      │
      ├──> listing::sort_entries()  ──> tri en place
      │
      ├──> display::display_short() ou display_long()
      │
      └──> (si -R) recursion sur les sous-repertoires
```
