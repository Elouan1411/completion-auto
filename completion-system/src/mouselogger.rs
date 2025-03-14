use byteorder::{NativeEndian, ReadBytesExt};
use std::{
    fs::OpenOptions,
    io::{Cursor, Read},
    path::Path,
};
use udev::Enumerator;

/// Reads mouse click events from the given file path.
///
/// This function opens the file and reads events to detect left mouse clicks.
/// If a left click is detected, it returns `Some(1)`.
///
/// # Arguments
///
/// * `path` - The path to the input device file.
///
/// # Returns
///
/// * `Option<u8>` - Returns `Some(1)` if a left click is detected, otherwise `None`.
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
            return Some(1); // Retourne 1 pour un clic gauche
        }
    }
}

/// Lists all mice and touchpads connected to the system.
///
/// This function scans the system for input devices and returns a list of paths
/// to the devices that are either mice or touchpads.
///
/// # Returns
///
/// * `Vec<String>` - A list of device paths for mice and touchpads.
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
