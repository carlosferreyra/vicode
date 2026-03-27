# Vicode (`vic`) 🚀

[![Crates.io](https://img.shields.io/crates/v/vicode.svg)](https://crates.io/crates/vicode)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust: 2024](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)

**Validated Infrastructure-from-Code (IfC) framework.** Define cloud resources directly in your
application code with compile-time safety.

> [!WARNING] This project is currently in the **planning and early development phase**. The features
> and architecture described below are being actively developed and are subject to change.

---

## 💡 What is Vicode?

**Vicode** is an **Infrastructure-from-Code (IfC)** framework written in Rust, designed to eliminate
the friction between application logic and cloud resource management.

Unlike traditional Infrastructure as Code (IaC) tools—like Terraform or Pulumi—where validation
often happens during the "plan" or "apply" phase, Vicode treats infrastructure as an **intrinsic
property of the source code**. It leverages the Rust type system and an agnostic Intermediate
Representation (IR) to validate your cloud topology **at compile-time**.

### ✨ Key Features

- **🛡️ Validated at Compile-time:** If your infrastructure graph is invalid (circular dependencies,
  missing mandatory resources), the code won't compile. Period.
- **🌐 Truly Agnostic:** Define abstract resources (Compute, Database, Storage) once and let
  **Vicode Drivers** translate your intent to AWS, Azure, GCP, or local environments.
- **🏎️ Minimalist CLI (`vic`):** A blazing-fast command-line tool written in Rust to manage your
  infrastructure lifecycle.
- **🔌 Polyglot Ready:** Designed with native bindings (PyO3/napi-rs) so Python (FastAPI/Django) and
  Node.js (Next.js/NestJS) developers can enjoy the same level of safety.

---

## 🛠️ Installation

Install the Vicode CLI globally using Cargo:

```bash
cargo install vicode
```
