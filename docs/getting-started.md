# Guide de Demarrage

## Prerequis

- **Rust** 1.85+ avec edition 2024
- Un systeme **Unix/macOS** (utilise `std::os::unix::fs::MetadataExt`)

## Installation

```bash
git clone <url-du-repo>
cd ft_ls
cargo build
```

## Premiere Utilisation

```bash
# Lister le repertoire courant
cargo run

# Lister un repertoire specifique
cargo run -- /tmp

# Combiner des flags
cargo run -- -l -a -t /home
```

## Utilisation avec le Makefile

```bash
# Build + execution
make run ARGS="-l"
make run ARGS="-R -a src"

# Nettoyage
make clean
```

## Exemples

### Listing basique
```
$ ft_ls
Cargo.lock  Cargo.toml  Makefile  src  target
```

### Format long
```
$ ft_ls -l
-rw-r--r--   1     4902 Mar  8 18:01  Cargo.lock
-rw-r--r--   1      129 Mar  8 18:01  Cargo.toml
-rw-r--r--   1       95 Mar  8 18:02  Makefile
drwxr-xr-x   6      192 Mar  8 19:26  src
drwxr-xr-x   5      160 Mar  8 18:00  target
```

### Recursif avec fichiers caches
```
$ ft_ls -R -a src
src:
.  ..  display.rs  entry.rs  listing.rs  main.rs
```

### Tri par date, ordre inverse
```
$ ft_ls -t -r
target  Cargo.toml  Cargo.lock  Makefile  src
```
