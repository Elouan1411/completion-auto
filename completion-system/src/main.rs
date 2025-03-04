use std::collections::HashMap;
use std::path::Path;
use std::thread;
use std::time::Duration;

use libc::geteuid;
use std::sync::{mpsc, Arc, Mutex};
use uinput::event::keyboard;
use uinput::Device;

mod python_gui;

use python_gui::PythonGUI;

mod keylogger;
mod mouselogger;
mod offset;
mod virtual_input;

use std::sync::atomic::{AtomicBool, Ordering};
pub static RUNNING: AtomicBool = AtomicBool::new(true);

fn main() {
    if unsafe { geteuid() } != 0 {
        eprintln!("Ce programme nécessite des privilèges administrateur pour fonctionner.");
        eprintln!("Il doit surveiller le clavier et la souris afin d'offrir l'auto-complétion.");
        eprintln!("Veuillez le relancer avec 'sudo'.");
        std::process::exit(1);
    }
    // init uinput
    let device: Device = virtual_input::init_virtual_key();

    // init keylogger
    let mut is_qwerty: bool = true;
    let keycode_map: HashMap<u16, String> = keylogger::init_keylogger(&mut is_qwerty);

    let keycode_uinput: HashMap<char, keyboard::Key> =
        virtual_input::create_keycode_uinput(is_qwerty);

    // init gui
    let device = Arc::new(Mutex::new(device));
    let keycode_uinput = Arc::new(Mutex::new(keycode_uinput));
    let gui = PythonGUI::new(Arc::clone(&device), Arc::clone(&keycode_uinput));

    // Récupération des chemins des périphériques d'entrée (claviers)
    let keyboard_paths: Vec<String> = keylogger::list_keyboards();
    // Récupération des chemins des périphériques d'entrée (souris)
    let mouse_paths: Vec<String> = mouselogger::list_mice_and_touchpads();

    let (tx, rx) = mpsc::channel(); // Canal de communication entre les threads
    let rx = Arc::new(Mutex::new(rx)); // Permet de partager `Receiver` entre plusieurs threads

    let mut handles = vec![];

    // Pour chaque chemin dans `keyboard_paths`, lancer un thread
    for path_str in keyboard_paths {
        let keycode_map = keycode_map.clone();
        let path = Path::new(&path_str).to_path_buf();
        let rx = Arc::clone(&rx);
        let gui_clone = gui.clone();

        let handle = thread::spawn(move || {
            let mut word: String = String::new();
            while RUNNING.load(Ordering::Relaxed) {
                if let Some(mut letter) = keylogger::get_pressed_key(&path, &keycode_map) {
                    letter = letter.to_lowercase();

                    if let Ok(rx) = rx.lock() {
                        if rx.try_recv().is_ok() {
                            word.clear();
                            offset::reset();
                            println!("🧹 Mot effacé à cause d'un clic !");

                            // Enlever tout les cliques qui sont dans la queue
                            while rx.try_recv().is_ok() {}
                        }
                    }

                    offset::manage_word(&mut letter, &mut word);
                    println!("⌨️ Clavier : {}", word);
                    gui_clone.send_words([word.as_str(), word.as_str(), word.as_str()]);
                }

                thread::sleep(Duration::from_millis(10));
            }
            println!("keyboard fermé");
        });

        handles.push(handle);
    }
    // Ggestion des souris

    // Pour chaque chemin dans `mouse_paths`, lancer un thread
    for path_str in mouse_paths {
        let path = Path::new(&path_str).to_path_buf();
        let tx = tx.clone();

        let handle = thread::spawn(move || {
            while RUNNING.load(Ordering::Relaxed) {
                if let Some(button) = mouselogger::get_mouse_click(&path) {
                    if button == 1 {
                        println!("🖱️ Souris : Clic gauche détecté !");
                        let _ = tx.send(()); // Envoie un signal au clavier pour effacer `word`
                                             // if let Ok(mut device) = device_clone.lock() { // TODO:supprimer de l'afffichage les mots de l'interface graphique une fois le clique dessus (pas forcement a traiter ici) plutot dans le programme python
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
    println!("✅ Programme terminé proprement !");
}
