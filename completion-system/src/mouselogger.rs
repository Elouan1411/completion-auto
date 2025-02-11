use byteorder::{NativeEndian, ReadBytesExt};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use udev::Enumerator;

pub fn get_mouse_click(path: &Path, button_map: &HashMap<u16, u8>) -> Option<u8> {
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
            return None;
        }

        let mut rdr = Cursor::new(packet);
        let _tv_sec = rdr.read_u64::<NativeEndian>().ok();
        let _tv_usec = rdr.read_u64::<NativeEndian>().ok();
        let evtype = rdr.read_u16::<NativeEndian>().ok();
        let code = rdr.read_u16::<NativeEndian>().ok();
        let value = rdr.read_i32::<NativeEndian>().ok();

        if evtype == Some(1) && value == Some(1) {
            // Vérifie si c'est un événement de type bouton de souris (EV_KEY)
            if let Some(&button_number) = button_map.get(&code.unwrap()) {
                println!("bouton de la souris : {}", button_number);
                return Some(button_number);
            } else {
                eprintln!(
                    "Chemin {:?} : Clic inconnu détecté (code {})",
                    path,
                    code.unwrap_or(0)
                );
            }
        }
    }
}

pub fn list_mice() -> Vec<String> {
    let mut devices = Vec::new();

    if let Ok(mut enumerator) = Enumerator::new() {
        if enumerator.match_subsystem("input").is_ok() {
            if let Ok(device_iter) = enumerator.scan_devices() {
                devices = device_iter
                    .filter_map(|device| {
                        if let Some(properties) = device.property_value("ID_INPUT_MOUSE") {
                            if properties == "1" {
                                if let Some(devnode) = device.devnode() {
                                    let path = devnode.to_string_lossy().into_owned();
                                    // On garde uniquement les périphériques de type eventX
                                    if path.contains("/event") {
                                        return Some(path);
                                    }
                                }
                            }
                        }
                        None
                    })
                    .collect();
            }
        }
    }

    devices
}

pub fn test() {
    let mice = list_mice();
    if mice.is_empty() {
        println!("🖱️ Aucune souris détectée !");
    } else {
        println!("🖱️ Souris détectées :");
        for mouse in mice {
            println!("- {}", mouse);
        }
    }
}
