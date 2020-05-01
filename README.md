# Reinforced Unity (Rust Utility)
Initializes Unity project directory with common defaults


## Manual

Requires the latest stable [Rust] compiler.

#### Building from source
```console
git clone https://github.com/Pineapple/ru.git
cd ru
cargo build --release
```

#### Installation
```console
cargo install --git https://github.com/Pineapple/ru.git
```

[rust]: https://www-rust-lang.org

#### Basic usage

This will create `foo` folder (if it doesn't exist) and initialize default project structure (`Assets` folder, `.gitignore`, `.gitattributes`).

```shell
$ ru init foo
```
