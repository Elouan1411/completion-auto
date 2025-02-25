use std::fs::{remove_file, File};
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

use std::collections::HashMap;

use uinput::event::keyboard;
use uinput::Device;

use crate::virtual_input;

const PYTHON_SCRIPT: &[u8] = include_bytes!("gui.py"); // Script Python embarqué

#[derive(Clone)]
pub struct PythonGUI {
    child: Arc<Mutex<std::process::Child>>,
    stdin: Arc<Mutex<std::process::ChildStdin>>,
}

impl PythonGUI {
    pub fn new(
        device: Arc<Mutex<Device>>,
        keycode_uinput: Arc<Mutex<HashMap<char, keyboard::Key>>>,
    ) -> Self {
        let temp_path = std::env::current_dir()
            .unwrap()
            .join("interface_embedded.py");

        let mut temp_file =
            File::create(&temp_path).expect("Impossible de créer le fichier temporaire");
        std::io::Write::write_all(&mut temp_file, PYTHON_SCRIPT)
            .expect("Impossible d'écrire le script");

        let mut child = Command::new("python3")
            .arg(&temp_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Échec du lancement de Python");

        let stdin = child.stdin.take().expect("Erreur accès stdin");
        let stdout = child.stdout.take().expect("Erreur accès stdout");
        let reader = BufReader::new(stdout);

        let child_arc = Arc::new(Mutex::new(child));
        let stdin_arc = Arc::new(Mutex::new(stdin));

        let device_clone = Arc::clone(&device);
        let keycode_clone = Arc::clone(&keycode_uinput);

        // ✅ Démarrer le thread après avoir créé l'instance
        thread::spawn(move || {
            for line in reader.lines() {
                match line {
                    Ok(msg) => {
                        if msg == "EXIT" {
                            println!("Python a demandé l'arrêt. Fermeture...");
                            let temp_path = std::env::current_dir()
                                .unwrap()
                                .join("interface_embedded.py");
                            println!("Suppression du fichier temporaire...");
                            let _ = remove_file(temp_path);
                            std::process::exit(0);
                        }
                        println!("Python a envoyé: {}", msg);

                        let mut device = device_clone.lock().unwrap();
                        let keycode_map = keycode_clone.lock().unwrap();
                        virtual_input::delete_and_write(msg, &mut *device, &*keycode_map);
                    }
                    Err(e) => eprintln!("Erreur lecture stdout: {}", e),
                }
            }
        });
        Self {
            child: child_arc,
            stdin: stdin_arc,
        }
    }

    /// Envoie trois mots à l'interface Python
    pub fn send_words(&self, words: [&str; 3]) {
        let mut stdin = self.stdin.lock().unwrap();
        writeln!(stdin, "{} {} {}", "Bonjour", words[1], words[2]).expect("Erreur écriture stdin");
    }
}

// Nettoyage automatique à la fermeture
impl Drop for PythonGUI {
    fn drop(&mut self) {
        let _ = self.child.lock().unwrap().kill();
        let temp_path = std::env::current_dir()
            .unwrap()
            .join("interface_embedded.py");
        println!("Suppression du fichier temporaire...");
        let _ = remove_file(temp_path);
    }
}
