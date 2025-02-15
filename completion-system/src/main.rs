use std::collections::HashMap;
use std::path::Path;
use std::thread;
use std::time::Duration;

use std::sync::{mpsc, Arc, Mutex};
use uinput::event::keyboard;
use uinput::Device;

mod keylogger;
mod mouselogger;
mod virtual_input;

fn main() {
    mouselogger::test();

    // init uinput
    let mut device: Device = virtual_input::init_virtual_key();

    // init keylogger
    let mut is_qwerty: bool = true;
    let keycode_map: HashMap<u16, String> = keylogger::init_keylogger(&mut is_qwerty);

    let keycode_uinput: HashMap<char, keyboard::Key> =
        virtual_input::create_keycode_uinput(is_qwerty);

    virtual_input::write_word("coucou Noopyé".to_string(), &mut device, &keycode_uinput);
    virtual_input::delete_word("Noopyé".to_string(), &mut device, &keycode_uinput);
    virtual_input::write_word("mon coeur".to_string(), &mut device, &keycode_uinput);

    // println!("\nLancement de la détection des touches !");

    // Récupération des chemins des périphériques d'entrée (claviers)
    let keyboard_paths: Vec<String> = keylogger::list_keyboards();
    // Récupération des chemins des périphériques d'entrée (souris)
    let mouse_paths: Vec<String> = mouselogger::list_mice_and_touchpads();

    let (tx, rx) = mpsc::channel(); // Canal de communication entre les threads
    let rx = Arc::new(Mutex::new(rx)); // Permet de partager `Receiver` entre plusieurs threads

    let mut handles = vec![];

    // Pour chaque chemin dans `keyboard_paths`, lancer un thread
    for path_str in keyboard_paths {
        let keycode_map = keycode_map.clone(); // Clone le keycode_map pour chaque thread
        let path = Path::new(&path_str).to_path_buf(); // Crée une copie du chemin pour chaque thread
        let rx = Arc::clone(&rx); // Clone l'Arc pour partager `rx`

        let handle = thread::spawn(move || {
            let mut word: String = String::new();
            let mut offset: usize = 0;
            loop {
                if let Some(mut letter) = keylogger::get_pressed_key(&path, &keycode_map) {
                    letter = letter.to_lowercase();

                    if let Ok(rx) = rx.lock() {
                        if rx.try_recv().is_ok() {
                            word.clear();
                            offset = 0;
                            println!("🧹 Mot effacé à cause d'un clic !");

                            // 🔥 Purger tous les événements restants dans la queue
                            while rx.try_recv().is_ok() {}
                        }
                    }

                    // Gestion du mot
                    if letter == "backspace" {
                        if offset >= 1 {
                            word.remove(offset - 1);
                            offset -= 1;
                        }
                    } else if letter == "left" {
                        if offset >= 1 {
                            offset -= 1;
                        }
                    } else if letter == "right" {
                        offset += 1;
                    }
                    // Vérifier si la lettre contient un seul caractère et si ce caractère est alphabétique
                    else if letter.chars().count() == 1 {
                        if let Some(first_char) = letter.chars().next() {
                            if first_char.is_alphabetic() || "éèàùç'".contains(first_char) {
                                word.insert(offset, first_char);
                                offset += 1;
                            } else {
                                // Si ce n'est pas une lettre, on vide le mot
                                word.clear();
                                offset = 0;
                            }
                        }
                    } else {
                        word.clear();
                        offset = 0;
                    }

                    println!("⌨️ Clavier : {}", word);
                }

                thread::sleep(Duration::from_millis(10));
            }
        });

        handles.push(handle);
    }

    // Ajout de la gestion des souris

    // Pour chaque chemin dans `mouse_paths`, lancer un thread
    for path_str in mouse_paths {
        let path = Path::new(&path_str).to_path_buf(); // Crée une copie du chemin pour chaque thread
        let tx = tx.clone(); // Clone le sender pour l'envoyer à chaque thread

        let handle = thread::spawn(move || {
            loop {
                if let Some(button) = mouselogger::get_mouse_click(&path) {
                    if button == 1 {
                        println!("🖱️ Souris : Clic gauche détecté !");
                        let _ = tx.send(()); // Envoie un signal au clavier pour effacer `word`
                    }
                }
                thread::sleep(Duration::from_millis(10));
            }
        });

        handles.push(handle);
    }

    // Attendre la fin de tous les threads avant de quitter
    for handle in handles {
        handle.join().expect("Le thread a rencontré une erreur.");
    }
}
