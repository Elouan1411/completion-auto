# **🔹 Description du projet**

Le projet vise à développer un **système de complétion semi-automatique** qui propose des suggestions de mots en temps réel à mesure que l'utilisateur tape.

Ce système repose sur :

- Un **keylogger** qui capture les frappes clavier et reconstitue le mot en cours.
- Un **mouselogger** qui détecte les clics pour réinitialiser le suivi du mot.
- Un **module `uinput`** qui permet d'insérer automatiquement un mot suggéré en simulant une frappe clavier réelle.

L'objectif est de faciliter la saisie en proposant des suggestions dynamiques et en permettant à l'utilisateur de les sélectionner rapidement sans effort supplémentaire.

---

# **🔹 Fonctionnement détaillé**

### **1️⃣ Keylogger : Capture et suivi du mot**

Le keylogger intercepte les frappes en lisant directement les événements clavier du système. Il détecte **chaque touche pressée**, traduit son code en caractère et construit le mot au fur et à mesure.

- Si l'utilisateur tape une lettre, elle est ajoutée au mot suivi.
- Si l'utilisateur appuie sur **Backspace**, la dernière lettre est supprimée.
- **Dès qu'une nouvelle lettre est ajoutée**, le système met à jour les suggestions en temps réel.

### **2️⃣ Mouselogger : Détection des clics**

Le mouselogger surveille les **clics gauche** de la souris ou du trackpad. Lorsqu'un clic est détecté, cela signifie que l'utilisateur a quitté la saisie du mot en cours. Dans ce cas, le suivi du mot est **réinitialisé** pour recommencer une nouvelle analyse.

### **3️⃣ Génération des suggestions**

À chaque mise à jour du mot suivi, un algorithme de complétion génère une liste de suggestions basées sur la correspondance avec un dictionnaire ou un modèle de prédiction. Ces suggestions sont affichées dans une interface graphique intuitive.

L'utilisateur peut sélectionner une suggestion en **cliquant dessus**.

### **4️⃣ `uinput` : Insertion automatique de la suggestion**

Lorsqu'une suggestion est choisie, le système **remplace le mot en cours** par celui sélectionné en utilisant `uinput`.

- D'abord, `uinput` **simule des appuis sur Backspace** pour supprimer le mot en cours.
- Ensuite, il **tape automatiquement** le mot suggéré lettre par lettre, comme si l'utilisateur l'écrivait lui-même.

Cette approche garantit une intégration fluide et naturelle dans n'importe quelle application de saisie.

---

# **🔹 Gestion des périphériques multiples**

Le système est conçu pour fonctionner avec **plusieurs claviers et souris**, permettant une compatibilité optimale avec les différents périphériques branchés.

Que l'utilisateur tape sur un **clavier externe, un clavier d'ordinateur portable** ou utilise un **trackpad ou une souris**, le comportement reste **cohérent et fluide**.
