# Technical Documentation — Auto-Completion

## General Description

This project implements a semi-automatic text completion tool that can be used in any application on an Ubuntu system.

## Installation

### Prerequisites

-   **Ubuntu** system (tested on Ubuntu 22.04.2 LTS)
-   The keyboard shortcut **Alt + Tab** must be enabled (default in Ubuntu) to switch between applications easily.

### Installation Steps

```bash
cd completion-auto/completion-system
make install
```

The `Makefile` takes care of:

-   Installing necessary dependencies
-   Copying the compiled binary to `/usr/local/bin/completion-system` (already in the PATH)
-   Setting the correct execution permissions for all required files

## Launching

Two methods are available:

-   **From the terminal**:

```bash
completion-system
```

-   **From the Ubuntu application launcher**:
    Open the menu, search for `Completion System`, and press Enter.

**Note:** On some systems, launching from the application launcher may not work. In that case, launching from the terminal works fully.
Once launched, the program runs in the background and works across **all applications**.

## Uninstallation

```bash
cd completion-auto/completion-system
make uninstall
```

## Standalone Binary

The compiled binary file `completion-system`, already provided in the `bin/` folder, is completely standalone after installation.

-   It is copied to `/usr/local/bin`, allowing it to be run from anywhere.
-   It no longer depends on source files or the Python script.

## Project Structure

The project directory structure is as follows:

```
.
|- data/
|  |- dico_freq.csv
|- src/
|  |- gui.py
|  |- keylogger.rs
|  |- main.rs
|  |- mouselogger.rs
|  |- offset.rs
|  |- python_gui.rs
|  |- suggestions.rs
|  |- virtual_input.rs
|- tools/
|  |- format_dico.py
|  |- Lexique383.tsv
|- Cargo.toml
|- completion-system.png
|- Makefile
|- README.md
```

### Folder and File Details

-   `src/` : Contains all Rust source files and the Python script for the graphical interface (`gui.py`).
-   `data/dico_freq.csv` : Final dictionary used for completion.
-   `tools/Lexique383.tsv` : Raw database file downloaded.
-   `tools/format_dico.py` : Python script used to convert the `.tsv` file into `.csv` and remove unnecessary information.
-   `Cargo.toml` : Configuration file specifying Rust dependencies and versions.
-   `completion-system.png` : Icon displayed in the Ubuntu application menu.
