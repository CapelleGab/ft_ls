# Modules Core

> Les 4 modules source de ft_ls

## Vue d'Ensemble

| Module | Fichier | Role |
|--------|---------|------|
| **Main** | `src/main.rs` | Parsing CLI, orchestration du pipeline |
| **Entry** | `src/entry.rs` | Structure de donnees `FileEntry` |
| **Listing** | `src/listing.rs` | Lecture repertoire, filtrage, tri |
| **Display** | `src/display.rs` | Formatage et affichage |

## Relations

```
main.rs
  ├── uses entry::FileEntry    (via listing)
  ├── uses listing::list_entries, sort_entries
  └── uses display::display_short, display_long

listing.rs
  └── uses entry::FileEntry

display.rs
  └── uses entry::FileEntry
```

## Documentation Detaillee

- [Main](./core/main.md) - Point d'entree et orchestration
- [Entry](./core/entry.md) - Structure FileEntry
- [Listing](./core/listing.md) - Lecture et tri des entrees
- [Display](./core/display.md) - Affichage des resultats
