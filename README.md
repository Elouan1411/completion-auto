# Word Auto-Completion Study

This repository explores the techniques and algorithms used in semi-automatic text completion. The project is part of our coursework for the CMI-L2 program and was completed in collaboration with Samia BENALI.

## Overview

When typing a message on a smartphone, suggestions for words are often provided to accelerate the process. These suggestions rely on efficient and relevant algorithms. This project aims to study and implement some of these techniques, focusing on:
1. Various approaches to text auto-completion.
2. Edit distance algorithms.
3. Markov chains for adapting to user history.

## Objectives

The project is divided into the following tasks:
1. **Study**: Review different approaches used in text auto-completion.
2. **Edit Distance Algorithms**: Understand and implement simple edit distance calculations.
3. **Markov Chains**: Study the basic principles and their use in adapting suggestions based on user history.
4. **Implementation**: Develop a small text auto-completion tool using the studied techniques.

## Contents

- **docs/**: Documentation and references.
- **src/**: Source code for the implementation.
- **tests/**: Unit tests for the algorithms and functionalities.
- **examples/**: Examples of how the tool works with various inputs.

## Requirements

This project uses Rust as the programming language.

### Prerequisites
1. Install Rust: Follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).
2. Ensure `cargo` (Rust's package manager) is installed.

### Build and Run
Navigate to the repository root and execute:
```bash
cargo build
cargo run
```

### Testing
Run the tests using:
```bash
cargo test
```

## Authors

- Elouan BOITEUX
- Samia BENALI
