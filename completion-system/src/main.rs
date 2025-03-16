use std::{
    collections::HashMap,
    path::Path,
    sync::{mpsc, Arc, Mutex},
};

use libc::geteuid;
use tokio_util::sync::CancellationToken;
use uinput::{event::keyboard, Device};

mod keylogger;
mod levenshtein;
mod mouselogger;
mod offset;
mod python_gui;
mod virtual_input;

use python_gui::PythonGUI;

#[tokio::main]
async fn main() {
    // // Example usage of get_suggestions
    // match levenshtein::get_suggestions("bambochiezz", "gutenberg.txt") {
    //     Ok(suggestions) => println!("Suggestions for 'bonkour': {:?}", suggestions),
    //     Err(e) => eprintln!("Error getting suggestions: {}", e),
    // }
    // std::process::exit(0);

    check_sudo();
    // init uinput
    let device: Device = virtual_input::init_virtual_key();

    // init keylogger
    let mut is_qwerty: bool = true;
    let keycode_map: HashMap<u16, String> = keylogger::init_keylogger(&mut is_qwerty);

    let keycode_uinput: HashMap<char, keyboard::Key> =
        virtual_input::create_keycode_uinput(is_qwerty);

    // init signal pour arreter le programme
    let token: Arc<CancellationToken> = Arc::new(CancellationToken::new());

    // init gui
    let device = Arc::new(Mutex::new(device));
    let keycode_uinput = Arc::new(Mutex::new(keycode_uinput));
    let gui = PythonGUI::new(
        Arc::clone(&device),
        Arc::clone(&keycode_uinput),
        token.clone(),
    );

    // Récupération des chemins des périphériques d'entrée (claviers)
    let keyboard_paths: Vec<String> = keylogger::list_keyboards();
    // Récupération des chemins des périphériques d'entrée (souris)
    let mouse_paths: Vec<String> = mouselogger::list_mice_and_touchpads();

    let (sender_canal, receiver_canal) = mpsc::channel(); // Canal de communication entre les threads
    let receiver_canal = Arc::new(Mutex::new(receiver_canal)); // Permet de partager `Receiver` entre plusieurs threads

    let mut handles = vec![];

    // Pour chaque chemin dans `keyboard_paths`, lancer un thread
    for path_str in keyboard_paths {
        let keycode_map = keycode_map.clone();
        let path = Path::new(&path_str).to_path_buf();
        let receiver_canal = Arc::clone(&receiver_canal);
        let gui_clone = gui.clone();
        let token_clone = token.clone();

        // Creation du thread pour chaque clavier
        let handle = tokio::spawn(async move {
            let mut word: String = String::new();
            tokio::select! {
                _ = token_clone.cancelled() => {}
                _ = async {
                    loop {
                        let path_clone = path.clone();
                        let keycode_map_clone = keycode_map.clone();
                        let letter = tokio::task::spawn_blocking(move || {
                            keylogger::get_pressed_key(&path_clone, &keycode_map_clone)
                        }).await.expect("Erreur lors de l'exécution de get_pressed_key");

                        if let Some(mut letter) = letter {
                            letter = letter.to_lowercase();

                            if let Ok(receiver_canal) = receiver_canal.lock() {
                                if receiver_canal.try_recv().is_ok() {
                                    // Effacer le mot en cours
                                    word.clear();
                                    offset::reset();
                                    // Enlever tout les cliques qui sont dans la queue
                                    while receiver_canal.try_recv().is_ok() {}
                                }
                            }

                            offset::manage_word(&mut letter, &mut word);
                            println!("⌨️ Clavier : {}", word);
                            // Envoie à l'interface graphique
                            gui_clone.send_words(["hello", "word", word.as_str()]);
                        }
                    }
                } => {}
            }
        });

        handles.push(handle);
    }
    // Pour chaque chemin dans `mouse_paths`, lancer un thread
    for path_str in mouse_paths {
        let path = Path::new(&path_str).to_path_buf();
        let sender_canal = sender_canal.clone();
        let token_clone = token.clone();

        // Création du thread
        let handle = tokio::spawn(async move {
            tokio::select! {
                _ = token_clone.cancelled() => {}
                _ = async {
                    loop {
                        // Récupère les évemenents de la souris et envoie un signal si il y a un clique gauche pour supprimer le mot
                        let path_clone = path.clone();
                        let button = tokio::task::spawn_blocking(move || {
                            mouselogger::get_mouse_click(&path_clone)
                        }).await.expect("Erreur lors de l'exécution de get_mouse_click");



                        if let Some(button) = button {
                            if button == 1 {
                                let _ = sender_canal.send(()); // Envoie un signal au clavier pour effacer `word`
                                // TODO:supprimer de l'afffexitage les mots de l'interface graphique une fois le clique dessus (pas forcement a traiter ici) plutot dans le programme python
                            }
                        }
                    }
                } => {}
            }
        });
        handles.push(handle);
    }

    // Attendre la fin de tous les threads avant de quitter
    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Erreur dans une tâche: {:?}", e);
        }
    }

    println!("✅ Programme terminé proprement !");
    std::process::exit(0); //TODO: essayer de l'enlever /
}

/// Checks if the program is running with sudo privileges.
///
/// If not, it prints an error message and exits the program.
fn check_sudo() {
    if unsafe { geteuid() } != 0 {
        eprintln!("Ce programme nécessite des privilèges administrateur pour fonctionner.");
        eprintln!("Il doit surveiller le clavier et la souris afin d'offrir l'auto-complétion.");
        eprintln!("Veuillez le relancer avec 'sudo'.");
        std::process::exit(1);
    }
}
