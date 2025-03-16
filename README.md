# Completion auto

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

<div align="center">
  <img src="https://github.com/user-attachments/assets/24ce49f6-1d3c-4cbf-bcc2-6232eaa5478e" width="50%"/>
</div>

## Contents

- **docs/**: Documentation and references.
- **completion-system/**: Main project directory containing the Rust and Python code.

## Requirements

This project uses Rust as the backend programming language and Python for the frontend.

### Prerequisites

1. Install dependencies:
   ```bash
   sudo apt update
   sudo apt install cargo libudev-dev pkg-config python3 python3-tk
   ```

### Build and Run

Execute:

```bash
cd completion-system
cargo build --release
sudo ./target/release/completion-system
```

> Note: The project must be executed with `sudo` because it needs to read peripheral actions and includes a virtual keyboard.

## Authors

- Elouan BOITEUX
- Samia BENALI
