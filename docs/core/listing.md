# Listing

> Lecture de repertoire, filtrage et tri des entrees

## Informations

| Propriete | Valeur |
|-----------|--------|
| **Fichier** | `src/listing.rs` |
| **Module** | `crate::listing` |
| **Exporte** | `list_entries`, `sort_entries` |
| **Depend de** | `crate::entry::FileEntry` |

## Role

Responsable de la lecture du systeme de fichiers et de la transformation des entrees brutes en `Vec<FileEntry>` filtree et triee.

## Fonctions

### `list_entries(path, show_hidden)`

Lit le contenu d'un repertoire et retourne les entrees sous forme de `Vec<FileEntry>`.

```rust
pub fn list_entries(path: &Path, show_hidden: bool) -> Vec<FileEntry>
```

| Parametre | Type | Description |
|-----------|------|-------------|
| `path` | `&Path` | Chemin du repertoire a lire |
| `show_hidden` | `bool` | Si `true`, inclut les fichiers commencant par `.` |

**Comportement :**
1. Appelle `fs::read_dir(path)` — affiche une erreur sur stderr en cas d'echec
2. Pour chaque entree, recupere les metadonnees via `fs::symlink_metadata()` (ne suit pas les liens symboliques)
3. Construit un `FileEntry` pour chaque entree valide
4. Filtre les fichiers caches si `show_hidden` est `false`
5. Trie alphabetiquement par nom (case-insensitive)

### `sort_entries(entries, by_time, reverse)`

Applique un tri supplementaire sur les entrees deja triees par nom.

```rust
pub fn sort_entries(entries: &mut Vec<FileEntry>, by_time: bool, reverse: bool)
```

| Parametre | Type | Description |
|-----------|------|-------------|
| `entries` | `&mut Vec<FileEntry>` | Entrees a trier en place |
| `by_time` | `bool` | Si `true`, tri par date de modification (recent d'abord) |
| `reverse` | `bool` | Si `true`, inverse l'ordre |

**Comportement :**
- Le tri par date ecrase le tri alphabetique par defaut
- L'inversion s'applique apres le tri (par date ou par nom)

## Details d'Implementation

### Pourquoi `symlink_metadata()` ?

`fs::metadata()` suit les liens symboliques et retourne les metadonnees de la cible. `fs::symlink_metadata()` retourne les metadonnees du lien lui-meme, ce qui est le comportement attendu de `ls` (afficher `l` comme type de fichier pour les symlinks).

### Tri case-insensitive

```rust
entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
```

Reproduit le comportement de `ls` qui ne distingue pas majuscules et minuscules dans le tri.

## Voir Aussi

- [Entry](./entry.md) - Structure FileEntry
- [Display](./display.md) - Affichage des entrees
