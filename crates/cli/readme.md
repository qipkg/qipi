<img src="../../.github/logo.png" width="140" align="right" />

# Qipi CLI

> 🧰 Command-line interface for the [Qipi](https://github.com/qipkg/qipi) package manager.

This crate contains the **main binary** for Qipi.

**Located at:** `crates/cli`

---

## 📂 Structure

```text
📦crates/cli
 ┣ 📂src
 ┃ ┣ 📂commands        # All CLI subcommands (init, install, add, etc.)
 ┃ ┃ ┣ 📜add.rs
 ┃ ┃ ┣ 📜init.rs
 ┃ ┃ ┣ 📜install.rs
 ┃ ┃ ┣ 📜list.rs
 ┃ ┃ ┣ 📜lock.rs
 ┃ ┃ ┣ 📜mount.rs
 ┃ ┃ ┣ 📜new.rs
 ┃ ┃ ┣ 📜remove.rs
 ┃ ┃ ┣ 📜shell.rs
 ┃ ┃ ┣ 📜umount.rs
 ┃ ┃ ┗ 📜uninstall.rs
 ┃ ┣ 📜commands.rs      # Command dispatcher
 ┃ ┣ 📜macros.rs        # CLI-related utility macros
 ┃ ┗ 📜main.rs          # Entry point
 ┣ 📜Cargo.toml
 ┗ 📜readme.md
```

---

## 🚀 Usage

Build and run the CLI:

```bash
cargo run -- -h
```

Or install globally:

```bash
cargo install --path crates/cli
```

Then use it as:

```bash
qp -h
```

---

## 📄 License

Licensed under the [MIT License](../../license).