import tkinter as tk
import threading
import sys

# Créer un événement d'arrêt global
stop_event = threading.Event()


def listen_rust():
    """Écoute Rust en continu et met à jour l'interface."""
    while not stop_event.is_set():
        line = sys.stdin.readline().strip()
        if not line:
            continue

        words = line.split()
        update_buttons(words)


def send_to_rust(word):
    """Envoie le mot cliqué à Rust."""
    print(word, flush=True)
    update_buttons(["" * 3])


def update_buttons(words):
    """Met à jour les boutons avec les nouveaux mots."""
    for widget in frame.winfo_children():
        widget.destroy()

    for i, word in enumerate(words):
        btn = tk.Button(
            frame,
            text=word,
            font=("Helvetica", 18),
            command=lambda w=word: send_to_rust(w),
            bd=0,  # Pas de bordure
            highlightthickness=0,
            relief=tk.FLAT,
            bg="#f5f5f5",  # Couleur de fond douce
            activebackground="#f5f5f5",
            padx=15,
            pady=10,
        )

        btn.pack(side=tk.LEFT, padx=(10 if i != 0 else 0, 10))

        # Ajout d'un séparateur | sauf après le dernier mot
        if i < len(words) - 1:
            separator = tk.Label(
                frame, text="|", font=("Helvetica", 18), fg="gray", bg="#f5f5f5"
            )
            separator.pack(side=tk.LEFT)


def on_close():
    """Quand la fenêtre est fermée, on envoie 'EXIT' à Rust et arrête le thread."""
    print("E X I T", flush=True)  # Informe Rust d'arrêter
    stop_event.set()  # Déclenche l'événement d'arrêt
    root.destroy()  # Ferme la fenêtre GUI
    sys.exit(0)  # Quitte immédiatement le script Python


# Création de l'interface Tkinter
root = tk.Tk()
root.title("Suggestions")

root.configure(bg="#f5f5f5")

root.minsize(400, 80)

# Garder la fenêtre toujours en premier plan
root.attributes("-topmost", True)

frame = tk.Frame(root, bg="#f5f5f5")
frame.pack(pady=20)


root.protocol("WM_DELETE_WINDOW", on_close)  # Capture le clic sur la croix

# Lancer l'écoute de Rust en arrière-plan
threading.Thread(target=listen_rust, daemon=True).start()

# Lancer l'interface graphique
root.mainloop()

