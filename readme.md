<img src="./.github/logo.png" width="140" align="right" />

# Qipi 🦉

> [!WARNING] 
> **Qipi** is in the development stage. It is not ready for any use at this time. ⏰

---

**Qipi** is a next-gen package manager for Node.js, built in Rust, designed for _extremely speed_, _cleanliness_, and _zero bloat_.

## 🚀 Features

- 📁 **No `node_modules`** — no clutter, no recursive resolution, no legacy tree.
- ⚡ **Extremely-fast installs** — projects install in under **1 second**.
- 🔁 **Single global store** — one install per version, forever.
- 📦 **Clean repos** — only `package.json` and `package.lock`. Nothing else.
- 🧠 **O(1) resolver** — powered by memory-mapped binary lockfiles & perfect hashing.
- 🔌 **Full compatibility** — seamless with all Node.js tooling. FUSE-based virtual `node_modules` when needed.
- 🧩 **Built-in workspaces** — simple, plug-and-play.

> [[Read the introductory and technical post]](https://nh3.pages.dev/blog/introducing-qipi)

---

## 📦 Quick Usage

```bash
# Create and enter a project
qp new app
cd app

# Add dependencies (done in <100ms)
qp add lodash

# Start a shell with Qipi's resolver
qp shell
node .
````

_Need legacy support?_

```bash
qp mount   # creates a virtual node_modules (FUSE)
qp umount  # removes it
```

---

## 🚀 Install

```bash
npm install -g qipi
```

You can find more installation methods [here](https://qipi.pages.dev/docs/install).

---

## 📚 Documentation

All **Qipi** documentation is located on the [official website](https://qipi.pages.dev). You'll find information about usage, architecture, FAQs, the community, and much more!

---

## ❤️ Contribute

**Any contribution is welcome!** You can contribute by reporting bugs, recommending features, contributing code, sharing on social media, and many other ways. Read the [contributing guidelines](./contributing.md) for more information.

---

## 📄 License

Licensed under the [MIT License](./license.md).

---

<div align="right">

#### Logo by [Camilo Zuluaga](https://github.com/camilo-zuluaga) ❤

</div>
