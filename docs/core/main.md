# Main

> Point d'entree et orchestration du programme

## Informations

| Propriete | Valeur |
|-----------|--------|
| **Fichier** | `src/main.rs` |
| **Module** | `crate` (racine) |
| **Dependances** | `clap`, `display`, `listing` |

## Role

`main.rs` est le point d'entree du programme. Il :
1. Definit la struct `Args` pour le parsing CLI avec `clap`
2. Orchestre le pipeline via la fonction `run()`
3. Gere le mode recursif par rappel de `run()`

## Structure `Args`

```rust
#[derive(Parser, Debug)]
#[command(name = "ft_ls", version, about = "List all files in directory like unix")]
struct Args {
    #[arg(default_value = ".")]
    destination: Vec<String>,
    #[arg(short = 'l', long)]
    long_format: bool,
    #[arg(short = 'a', long)]
    show_hidden: bool,
    #[arg(short = 'r', long)]
    rev: bool,
    #[arg(short = 't', long)]
    order_by_update_date: bool,
    #[arg(short = 'R', long)]
    recursive: bool,
}
```

| Champ | Type | Flag | Description |
|-------|------|------|-------------|
| `destination` | `Vec<String>` | positional | Chemins a lister (defaut : `.`) |
| `long_format` | `bool` | `-l` | Active le format long |
| `show_hidden` | `bool` | `-a` | Affiche les fichiers caches |
| `rev` | `bool` | `-r` | Inverse l'ordre de tri |
| `order_by_update_date` | `bool` | `-t` | Tri par date de modification |
| `recursive` | `bool` | `-R` | Mode recursif |

## Fonctions

### `main()`

Point d'entree. Parse les arguments, itere sur les destinations et appelle `run()`.

Affiche un header (`nom_du_dossier:`) si plusieurs destinations sont fournies ou si le mode recursif est actif.

### `run(args, path, show_header)`

Orchestre le pipeline pour un chemin donne :

```rust
fn run(args: &Args, path: &Path, show_header: bool) {
    // 1. Affiche le header si necessaire
    // 2. Lit et filtre les entrees du repertoire
    // 3. Applique le tri (date, inverse)
    // 4. Affiche en format court ou long
    // 5. Si -R : recursion sur les sous-dossiers
}
```

| Parametre | Type | Description |
|-----------|------|-------------|
| `args` | `&Args` | Arguments parses |
| `path` | `&Path` | Chemin du repertoire a lister |
| `show_header` | `bool` | Afficher le nom du repertoire avant le listing |

## Voir Aussi

- [Entry](./entry.md) - Structure des entrees
- [Listing](./listing.md) - Lecture et tri
- [Display](./display.md) - Affichage
