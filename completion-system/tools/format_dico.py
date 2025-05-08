import csv
from collections import defaultdict

# Chemin vers le fichier TSV d'entrée et CSV de sortie
fichier_tsv = "Lexique383.tsv"
fichier_csv = "dico_freq.csv"


# Fonction pour convertir TSV en CSV en gardant la ligne avec la plus haute fréquence pour chaque mot
def convertir_tsv_en_csv(fichier_tsv, fichier_csv):
    # Dictionnaire pour garder la trace de la fréquence des valeurs de la huitième colonne pour chaque mot
    frequency_map = defaultdict(lambda: defaultdict(int))

    # Dictionnaire pour garder la meilleure ligne (avec la fréquence la plus haute)
    best_line_map = {}

    with open(fichier_tsv, "r", newline="") as tsvfile, open(
        fichier_csv, "w", newline=""
    ) as csvfile:
        tsvreader = csv.reader(tsvfile, delimiter="\t")
        csvwriter = csv.writer(csvfile, delimiter=",")

        # Ignorer la première ligne du fichier TSV (si c'est une en-tête)
        next(tsvreader)

        for ligne in tsvreader:
            if len(ligne) >= 8:
                # Extraire la première et la huitième colonne
                colonne_1 = ligne[0]
                colonne_8 = ligne[7]

                # Vérifier si l'une des colonnes contient des espaces
                if " " not in colonne_1 and " " not in colonne_8:
                    # Incrémenter la fréquence de la valeur dans la huitième colonne pour ce mot
                    frequency_map[colonne_1][colonne_8] += 1

                    # Si c'est la première occurrence de ce mot, ou si la fréquence est plus haute, on met à jour la ligne
                    if colonne_1 not in best_line_map or frequency_map[colonne_1][
                        colonne_8
                    ] > frequency_map[colonne_1].get(best_line_map[colonne_1][1], 0):
                        best_line_map[colonne_1] = [colonne_1, colonne_8]

        # Écrire les lignes avec la fréquence la plus haute, en ignorant la première ligne
        first_line_written = False  # Indicateur pour ignorer la première ligne

        for best_line in best_line_map.values():
            # Écrire la première ligne après avoir ignoré la première écriture
            if not first_line_written:
                first_line_written = True
                continue  # Ignorer la première ligne du fichier CSV

            # Écrire les lignes restantes
            csvwriter.writerow([best_line[0], best_line[1]])


# Appeler la fonction pour effectuer la conversion
convertir_tsv_en_csv(fichier_tsv, fichier_csv)
