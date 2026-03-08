# Architecture de ft_ls

## Vue d'Ensemble

ft_ls suit une architecture en **pipeline** simple et lineaire :

```
Parsing Args  →  Lecture Repertoire  →  Filtrage  →  Tri  →  Affichage
   (clap)         (listing.rs)       (listing.rs)  (listing.rs)  (display.rs)
```

## Modules

### `main.rs` - Orchestration

Point d'entree du programme. Responsabilites :
- Parse les arguments CLI via `clap::Parser`
- Itere sur les chemins fournis
- Appelle `run()` qui orchestre le pipeline pour chaque chemin
- Gere la recursion (`-R`) en rappelant `run()` sur les sous-repertoires

### `entry.rs` - Modele de Donnees

Definit `FileEntry`, la structure centrale qui encapsule :
- Le nom du fichier
- Son chemin complet (`PathBuf`)
- Ses metadonnees (`std::fs::Metadata`)

Fournit des methodes utilitaires : `is_dir()`, `is_hidden()`, `modified_time()`.

### `listing.rs` - Lecture et Tri

Deux fonctions publiques :
- `list_entries()` : lit un repertoire, filtre les fichiers caches, trie par nom
- `sort_entries()` : applique le tri par date (`-t`) et l'inversion (`-r`)

### `display.rs` - Affichage

Deux modes d'affichage :
- `display_short()` : noms separes par des espaces (mode par defaut)
- `display_long()` : format detaille avec permissions, liens, taille, date

Fonctions internes pour formater les permissions Unix et les dates.

## Flux de Donnees

```
main()
  │
  ├── Args::parse()           // Parse les arguments CLI
  │
  └── pour chaque destination:
        │
        └── run(&args, path)
              │
              ├── list_entries(path, show_hidden)
              │     │
              │     ├── fs::read_dir(path)         // Lecture du repertoire
              │     ├── FileEntry::new(path, meta)  // Construction des entrees
              │     ├── filter(is_hidden)            // Filtrage -a
              │     └── sort_by(name)                // Tri alphabetique par defaut
              │
              ├── sort_entries(&mut entries, by_time, reverse)
              │     ├── sort_by(modified_time)       // Tri -t
              │     └── reverse()                    // Inversion -r
              │
              ├── display_long() ou display_short()  // Affichage -l
              │
              └── si -R: pour chaque sous-dossier → run() recursif
```

## Dependances

| Crate | Version | Usage |
|-------|---------|-------|
| `clap` | 4.5.60 | Parsing des arguments CLI (feature `derive`) |

Aucune autre dependance externe. Le formatage des dates et des permissions est fait manuellement avec la bibliotheque standard.

## Decisions Techniques

- **`fs::symlink_metadata()`** au lieu de `fs::metadata()` : permet de lire les metadonnees des liens symboliques sans les suivre
- **Algorithme de date de Howard Hinnant** : conversion epoch → date civile sans dependance externe
- **Format de date conditionnel** : affiche l'annee si le fichier a plus de 6 mois, sinon affiche l'heure (comme `ls`)
- **Tri case-insensitive** : `to_lowercase()` pour le tri alphabetique par defaut
