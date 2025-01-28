use byteorder::{NativeEndian, ReadBytesExt};
use std::fs::OpenOptions;
use std::io::{Cursor, Read};
use std::path::Path;
use std::thread;
mod keylogger;
mod virtual_input;
use std::collections::HashMap;

use uinput::event::keyboard;
use uinput::Device;

fn main() {
    // init uinput
    let mut device: Device = virtual_input::init_virtual_key();

    // init keylogger
    let mut is_qwerty: bool = true;
    let keycode_map: HashMap<u16, String> = keylogger::init_keylogger(&mut is_qwerty);

    let keycode_uinput: HashMap<char, keyboard::Key> =
        virtual_input::create_keycode_uinput(is_qwerty);

    virtual_input::write_word("Bonjour Noopy".to_string(), &mut device, &keycode_uinput);

    println!("\nLancement de la détection des touches !");

    // Récupération des détails système avec les chemins des périphériques d'entrée

    let event_paths: Vec<String> = keylogger::list_keyboards();

    let mut handles = vec![];

    // Pour chaque chemin dans event_paths, lancer un thread
    for path_str in event_paths {
        let path = Path::new(&path_str).to_path_buf(); // Crée une copie du chemin pour chaque thread
        let keycode_map = keycode_map.clone(); // Clone le keycode_map pour chaque thread

        let handle = thread::spawn(move || {
            let mut file_options = OpenOptions::new();
            file_options.read(true).write(false).create(false);

            let mut event_file = match file_options.open(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Erreur lors de l'ouverture du fichier {:?} : {}", path, err);
                    return;
                }
            };

            let mut packet = [0u8; 24];
            loop {
                if let Err(err) = event_file.read_exact(&mut packet) {
                    eprintln!(
                        "Erreur lors de la lecture des données dans {:?} : {}",
                        path, err
                    );
                    break;
                }

                let mut rdr = Cursor::new(packet);
                let _tv_sec = rdr.read_u64::<NativeEndian>().unwrap();
                let _tv_usec = rdr.read_u64::<NativeEndian>().unwrap();
                let evtype = rdr.read_u16::<NativeEndian>().unwrap();
                let code = rdr.read_u16::<NativeEndian>().unwrap();
                let value = rdr.read_i32::<NativeEndian>().unwrap();

                if evtype == 1 && value == 1 {
                    // Vérifie si c'est un événement de type touche (EV_KEY)
                    match keycode_map.get(&code) {
                        Some(letter) => {
                            println!("{}", letter);
                        }
                        None => println!(
                            "Chemin {:?} : Aucune lettre trouvée pour le keycode {}",
                            path, code
                        ),
                    }
                }
            }
        });

        handles.push(handle);
    }

    // Attendre la fin de tous les threads avant de quitter
    for handle in handles {
        handle.join().unwrap();
    }
}
