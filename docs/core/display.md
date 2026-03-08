# Display

> Formatage et affichage des resultats

## Informations

| Propriete | Valeur |
|-----------|--------|
| **Fichier** | `src/display.rs` |
| **Module** | `crate::display` |
| **Exporte** | `display_short`, `display_long` |
| **Depend de** | `crate::entry::FileEntry`, `std::os::unix::fs::MetadataExt` |

## Role

Responsable de l'affichage des entrees dans le terminal. Supporte deux modes : format court (defaut) et format long (`-l`).

## Fonctions Publiques

### `display_short(entries)`

Affiche les noms des fichiers separes par deux espaces sur une seule ligne.

```rust
pub fn display_short(entries: &[FileEntry]) {
    let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    println!("{}", names.join("  "));
}
```

**Sortie :**
```
Cargo.lock  Cargo.toml  Makefile  src  target
```

### `display_long(entries)`

Affiche chaque entree sur une ligne avec les informations detaillees.

```rust
pub fn display_long(entries: &[FileEntry])
```

**Format de sortie :**
```
{permissions} {nlinks} {taille} {date}  {nom}
```

**Exemple :**
```
-rw-r--r--   1     4902 Mar  8 18:01  Cargo.lock
drwxr-xr-x   6      192 Mar  8 19:26  src
```

## Fonctions Internes

### `format_permissions(mode)`

Convertit un mode Unix (`u32`) en chaine lisible de 10 caracteres.

```rust
fn format_permissions(mode: u32) -> String
```

**Types de fichiers supportes :**

| Masque | Caractere | Type |
|--------|-----------|------|
| `0o040000` | `d` | Repertoire |
| `0o120000` | `l` | Lien symbolique |
| autre | `-` | Fichier regulier |

**Permissions :** Chaque triplet (owner, group, other) est decode bit par bit en `rwx` ou `-`.

### `format_time(time)`

Formate un `SystemTime` en chaine lisible, a la maniere de `ls`.

```rust
fn format_time(time: std::time::SystemTime) -> String
```

**Deux formats selon l'age du fichier :**
- **Moins de 6 mois** : `Mon DD HH:MM` (ex: `Mar  8 18:01`)
- **Plus de 6 mois** : `Mon DD  YYYY` (ex: `Sep 15  2024`)

### `days_to_date(days)`

Convertit un nombre de jours depuis l'epoch Unix en date civile `(annee, mois, jour)`.

```rust
fn days_to_date(days: i64) -> (i64, i64, i64)
```

Utilise l'algorithme de **Howard Hinnant** pour la conversion sans dependance externe. Le mois retourne est 0-indexed (0 = Janvier).

## Points d'Attention

- **Owner/Group** : Le format long actuel n'affiche pas le nom du proprietaire ni du groupe (seulement nlinks et taille)
- **Largeur des colonnes** : Les largeurs sont fixes (`{:>3}`, `{:>8}`), pas calculees dynamiquement comme dans le vrai `ls`
- **Total blocks** : Le vrai `ls -l` affiche `total N` avant le listing, ce qui n'est pas implemente

## Ameliorations Futures

- [ ] Afficher owner et group dans le format long
- [ ] Colonnes alignees dynamiquement
- [ ] Afficher `total` (nombre de blocks) en mode `-l`
- [ ] Colorisation des noms (dossiers en bleu, executables en vert, etc.)

## Voir Aussi

- [Entry](./entry.md) - Structure des donnees affichees
- [Architecture](../architecture.md) - Place dans le pipeline
