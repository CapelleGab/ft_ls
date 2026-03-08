# Entry

> Structure de donnees representant un fichier ou dossier

## Informations

| Propriete | Valeur |
|-----------|--------|
| **Fichier** | `src/entry.rs` |
| **Module** | `crate::entry` |
| **Exporte** | `FileEntry` |

## Role

Definit `FileEntry`, la structure centrale du projet. Elle encapsule un fichier avec son nom, son chemin complet et ses metadonnees systeme. Fournit des methodes utilitaires pour interroger les proprietes du fichier.

## Structure `FileEntry`

```rust
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub metadata: Metadata,
}
```

| Champ | Type | Description |
|-------|------|-------------|
| `name` | `String` | Nom du fichier (extrait du chemin) |
| `path` | `PathBuf` | Chemin complet du fichier |
| `metadata` | `Metadata` | Metadonnees systeme (permissions, taille, dates) |

## Methodes

### `FileEntry::new(path, metadata)`

Constructeur. Extrait le nom du fichier a partir du `PathBuf`.

```rust
pub fn new(path: PathBuf, metadata: Metadata) -> Self
```

### `is_dir()`

Retourne `true` si l'entree est un repertoire.

```rust
pub fn is_dir(&self) -> bool {
    self.metadata.is_dir()
}
```

### `is_hidden()`

Retourne `true` si le nom commence par `.` (convention Unix).

```rust
pub fn is_hidden(&self) -> bool {
    self.name.starts_with('.')
}
```

### `modified_time()`

Retourne la date de derniere modification. Retourne `UNIX_EPOCH` en cas d'erreur.

```rust
pub fn modified_time(&self) -> SystemTime {
    self.metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH)
}
```

## Utilisation

`FileEntry` est cree par `listing::list_entries()` et consomme par :
- `listing::sort_entries()` pour le tri
- `display::display_short()` et `display_long()` pour l'affichage
- `main::run()` pour la recursion (verification `is_dir()`)

## Voir Aussi

- [Listing](./listing.md) - Creation et tri des FileEntry
- [Display](./display.md) - Affichage des FileEntry
