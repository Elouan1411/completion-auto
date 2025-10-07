# Documentation technique — Complétion automatique

## Description générale

Ce projet implémente un outil de complétion semi-automatique de texte, utilisable dans toutes les applications sur un système Ubuntu.

## Installation

### Prérequis

- Système **Ubuntu** (testé sur Ubuntu 22.04.2 LTS)  
- Le raccourci clavier **Alt + Tab** doit être actif (natif sur Ubuntu) pour changer d'application facilement

### Étapes d'installation

```bash
cd completion-auto/completion-system
make install
````

Le `Makefile` s’occupe de :

* Installer les dépendances nécessaires
* Copier le binaire compilé dans `/usr/local/bin/completion-system` (inclus dans le PATH)
* Donner les bons droits d’exécution à tous les fichiers nécessaires

## Lancement

Deux méthodes sont possibles :

* **Depuis le terminal** :

```bash
completion-system
```

* **Depuis le gestionnaire d'applications Ubuntu** :
  Ouvrir le menu et rechercher `Completion System`, puis appuyer sur Entrée.

**Remarque :** sur certains ordinateurs, il est possible que le lancement via le gestionnaire d’applications ne fonctionne pas. Dans ce cas, le terminal reste pleinement fonctionnel.
Une fois lancé, le programme tourne en arrière-plan et fonctionne dans **toutes les applications**.

## Désinstallation

```bash
cd completion-auto/completion-system
make uninstall
```

## Binaire autonome

Le fichier binaire `completion-system`, déjà compilé et fourni dans le dossier `bin/`, est entièrement autonome après installation.

* Il est copié dans `/usr/local/bin`, ce qui permet de le lancer depuis n'importe où.
* Il ne dépend plus des fichiers sources ni du script Python.

## Structure des fichiers

L'arborescence du projet est la suivante :

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

### Détail des dossiers et fichiers

* `src/` : contient tous les fichiers source Rust ainsi que le script Python pour l'interface graphique (`gui.py`).
* `data/dico_freq.csv` : dictionnaire final utilisé pour la complétion.
* `tools/Lexique383.tsv` : base de données brute téléchargée.
* `tools/format_dico.py` : script Python utilisé pour convertir le fichier `.tsv` en `.csv`, et pour enlever les informations inutiles.
* `Cargo.toml` : fichier de configuration indiquant les dépendances Rust et leurs versions.
* `completion-system.png` : icône affichée dans le menu d’applications Ubuntu.

