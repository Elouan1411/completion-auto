use std::collections::HashMap;

use std::path::Path;
use std::thread;
use uinput::event::keyboard;
use uinput::Device;

mod keylogger;
mod virtual_input;

fn main() {
    // init uinput
    let mut device: Device = virtual_input::init_virtual_key();

    // init keylogger
    let mut is_qwerty: bool = true;
    let keycode_map: HashMap<u16, String> = keylogger::init_keylogger(&mut is_qwerty);

    let keycode_uinput: HashMap<char, Vec<keyboard::Key>> = virtual_input::create_keycode_uinput();

    virtual_input::write_word("é".to_string(), &mut device, &keycode_uinput);

    // println!("\nLancement de la détection des touches !");

    // Récupération des chemins des périphériques d'entrée
    let event_paths: Vec<String> = keylogger::list_keyboards();

    let mut handles = vec![];

    // Pour chaque chemin dans event_paths, lancer un thread=
    for path_str in event_paths {
        let path = Path::new(&path_str).to_path_buf(); // Crée une copie du chemin pour chaque thread
        let keycode_map = keycode_map.clone(); // Clone le keycode_map pour chaque thread

        let handle = thread::spawn(move || {
            let mut word: String = String::new();
            loop {
                if let Some(mut letter) = keylogger::get_pressed_key(&path, &keycode_map) {
                    letter = letter.to_lowercase(); // Convertir la lettre en minuscule

                    // Vérifier si la lettre contient un seul caractère et si ce caractère est alphabétique
                    if letter.chars().count() == 1 {
                        // Vérifier explicitement si le premier caractère est une lettre
                        if let Some(first_char) = letter.chars().next() {
                            if first_char.is_alphabetic() || "éèàùç'ù".contains(first_char) {
                                word.push_str(&letter); // Ajouter la lettre au mot
                            } else {
                                word.clear(); // Si ce n'est pas une lettre, on vide le mot
                            }
                        }
                        // TODO: détecter les clics de la souris pour clear aussi
                    } else {
                        word.clear(); // Si la lettre n'est pas une seule lettre, on vide le mot
                    }

                    println!("{}", word); // Affiche le mot formé
                }
                // Pour éviter une surcharge inutile du CPU
                thread::sleep(std::time::Duration::from_millis(10));
            }
        });

        handles.push(handle);
    }

    // Attendre la fin de tous les threads avant de quitter
    for handle in handles {
        handle.join().expect("Le thread a rencontré une erreur.");
    }
}
