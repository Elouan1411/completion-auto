use byteorder::{NativeEndian, ReadBytesExt};
use std::fs::OpenOptions;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;
use udev::Enumerator;

pub fn get_mouse_click(path: &Path) -> Option<u8> {
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

        // Vérifie si c'est un clic gauche (EV_KEY = 1, BTN_LEFT = 272, value = 1 pour un appui)
        if evtype == Some(1) && code == Some(272) && value == Some(1) {
            println!("Clic gauche détecté !");
            return Some(1); // Retourne 1 pour un clic gauche
        }
    }
}

pub fn list_mice_and_touchpads() -> Vec<String> {
    let mut devices = Vec::new();

    if let Ok(mut enumerator) = Enumerator::new() {
        if enumerator.match_subsystem("input").is_ok() {
            if let Ok(device_iter) = enumerator.scan_devices() {
                devices = device_iter
                    .filter_map(|device| {
                        let mut is_valid = false;

                        if let Some(properties) = device.property_value("ID_INPUT_MOUSE") {
                            if properties == "1" {
                                is_valid = true;
                            }
                        }

                        if let Some(properties) = device.property_value("ID_INPUT_TOUCHPAD") {
                            if properties == "1" {
                                is_valid = true;
                            }
                        }

                        if is_valid {
                            if let Some(devnode) = device.devnode() {
                                let path = devnode.to_string_lossy().into_owned();
                                if path.contains("/event") {
                                    return Some(path);
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
<<<<<<< HEAD
=======

pub fn test() {
    let mice = list_mice_and_touchpads();
    if mice.is_empty() {
        println!("🖱️ Aucune souris détectée !");
    } else {
        println!("🖱️ Souris détectées :");
        for mouse in mice {
            println!("- {}", mouse);
        }
    }
}
>>>>>>> f0f6e19d2469ca8129ceced3c085bb196125d6d2
