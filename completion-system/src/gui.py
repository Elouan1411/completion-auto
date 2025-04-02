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

def update_buttons(words):
    """Met à jour les boutons avec les nouveaux mots."""
    for widget in frame.winfo_children():
        widget.destroy()

    for word in words:
        btn = tk.Button(frame, text=word, font=("Arial", 14),
                        command=lambda w=word: send_to_rust(w))
        btn.pack(side=tk.LEFT, padx=5, pady=5)

def on_close():
    """Quand la fenêtre est fermée, on envoie 'EXIT' à Rust et arrête le thread."""
    print("E X I T", flush=True)  # Informe Rust d'arrêter
    stop_event.set()  # Déclenche l'événement d'arrêt
    root.destroy()  # Ferme la fenêtre GUI
    sys.exit(0)  # Quitte immédiatement le script Python

# Création de l'interface Tkinter
root = tk.Tk()
root.title("Interface Rust <-> Python")
root.minsize(400,50)

# Garder la fenêtre toujours en premier plan
root.attributes("-topmost", True)

frame = tk.Frame(root)
frame.pack(pady=20)

root.protocol("WM_DELETE_WINDOW", on_close)  # Capture le clic sur la croix

# Lancer l'écoute de Rust en arrière-plan
threading.Thread(target=listen_rust, daemon=True).start()

# Lancer l'interface graphique
root.mainloop()