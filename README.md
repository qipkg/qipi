<img src="./public/logo.png" width="150px" align="right" />

> [!WARNING] > **Qipi** is in the development stage. It is not ready for any use at this time. ⏰

# Qipi - Package Manager

🦉 **Qipi** is an extremely fast and disk-efficient package manager for NodeJS written in Rust.

## ✨ Features

- **Extremely fast:** Qipi is the fastest package manager: **20x the speed of PNPM**. See the [benchmarks](./benches).
- **Disk-efficient:** It uses a centralized cache architecture and symlinks to avoid duplicates. Install once for everything.
- **Secure:** Checks cryptographic signatures and prevents automatic post-install scripts.
- **Deterministic:** Maintain consistency between projects with a lockfile (`package.lock`)

## 🚀 Installation

Install **Qipi** with the following command:

```bash
npm install -g qipi
```

## 📈 Benchmarks

You can see the different benchmarks [here](./benches).

## 📚 Documentation

For detailed documentation, visit the [Qipi website](https://qipi.pages.dev).

## 🤝 Contributing

Contributions are welcome! Please see our [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on how to contribute.

## 📄 License

This project is licensed under the [MIT License](./LICENSE).

<hr />

<div align="right">

##### Thanks to [Camilo Zuluaga](https://github.com/camilo-zuluaga) for creating the logo. ❤

</div>
