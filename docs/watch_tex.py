import time
import subprocess
import os

TEX_FILE = "report.tex" 
COMMAND = ["texcompiler", TEX_FILE] 


def main():
    if not os.path.exists(TEX_FILE):
        print(f"Fichier introuvable : {TEX_FILE}")
        return

    last_mtime = os.path.getmtime(TEX_FILE)

    print(f"Surveillance de {TEX_FILE}... (Ctrl+C pour quitter)")
    try:
        while True:
            time.sleep(0.5)  # intervalle de vérification
            current_mtime = os.path.getmtime(TEX_FILE)
            if current_mtime != last_mtime:
                last_mtime = current_mtime
                print(f"\nFichier modifié à {time.ctime(current_mtime)}")
                try:
                    subprocess.run(COMMAND, check=True)
                except subprocess.CalledProcessError as e:
                    print("Erreur lors de l'exécution de la commande :", e)
    except KeyboardInterrupt:
        print("\nArrêt du script.")


if __name__ == "__main__":
    main()
