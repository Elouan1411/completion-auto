use crate::virtual_input;
use std::{
    collections::HashMap,
    fs::{remove_dir_all, set_permissions, File},
    io::{BufRead, BufReader, Write},
    os::unix::fs::PermissionsExt,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};
use tempfile::TempDir;
use tokio_util::sync::CancellationToken;
use uinput::{event::keyboard, Device};

const PYTHON_SCRIPT: &[u8] = include_bytes!("gui.py");

#[derive(Clone)]
pub struct PythonGUI {
    _child: Arc<Mutex<Option<std::process::Child>>>,
    stdin: Arc<Mutex<Option<std::process::ChildStdin>>>,
    _temp_dir: Arc<TempDir>,
}

impl PythonGUI {
    pub fn new(
        device: Arc<Mutex<Device>>,
        keycode_uinput: Arc<Mutex<HashMap<char, keyboard::Key>>>,
        token: Arc<CancellationToken>,
    ) -> Self {
        let temp_dir = Arc::new(
            tempfile::Builder::new()
                .prefix("completion_system_temp_")
                .tempdir()
                .expect("Impossible de créer un dossier temporaire"),
        );

        let temp_path = temp_dir.path();
        set_permissions(temp_path, std::fs::Permissions::from_mode(0o755))
            .expect("Impossible de définir les permissions");

        let script_path = temp_path.join("gui.py");
        let mut script_file = File::create(&script_path).expect("Impossible de créer le fichier");
        script_file
            .write_all(PYTHON_SCRIPT)
            .expect("Erreur d'écriture");

        let mut child = Command::new("python3")
            .arg(&script_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Échec du lancement de Python");

        let stdin = child.stdin.take();
        let stdout = child.stdout.take();
        let reader = BufReader::new(stdout.expect("Erreur accès stdout"));

        let child_arc = Arc::new(Mutex::new(Some(child)));
        let stdin_arc = Arc::new(Mutex::new(stdin));
        let device_clone = Arc::clone(&device);
        let keycode_clone = Arc::clone(&keycode_uinput);
        let temp_dir_clone = Arc::clone(&temp_dir);
        let child_clone = Arc::clone(&child_arc);
        let stdin_clone = Arc::clone(&stdin_arc);

        tokio::spawn(async move {
            for line in reader.lines() {
                match line {
                    Ok(msg) => {
                        if msg == "E X I T" {
                            // 🔹 Ferme `stdin` avant de tuer Python
                            if let Ok(mut stdin_lock) = stdin_clone.lock() {
                                *stdin_lock = None;
                            }

                            // 🔹 Tue le processus Python
                            if let Ok(mut child_lock) = child_clone.lock() {
                                if let Some(child) = child_lock.as_mut() {
                                    let _ = child.kill();
                                }
                                *child_lock = None;
                            }

                            let _ = remove_dir_all(temp_dir_clone.clone().path());

                            token.cancel();
                            // exit(0); //Temporaire
                            // break;
                        } else {
                            let mut device = device_clone.lock().unwrap();
                            let keycode_map = keycode_clone.lock().unwrap();
                            virtual_input::delete_and_write(msg, &mut *device, &*keycode_map);
                        }
                    }
                    Err(e) => eprintln!("Erreur lecture stdout: {}", e),
                }
            }
        });

        Self {
            _child: child_arc,
            stdin: stdin_arc,
            _temp_dir: temp_dir,
        }
    }

    /// Envoie trois mots à l'interface Python
    // pub fn send_words(&self, words: [&str; 3]) {
    //     if let Ok(mut stdin_lock) = self.stdin.lock() {
    //         if let Some(ref mut stdin) = *stdin_lock {
    //             if writeln!(stdin, "{} {} {}", words[0], words[1], words[2]).is_err() {
    //                 eprintln!("Erreur écriture stdin: Broken Pipe (Python fermé ?)");
    //             }
    //         }
    //     }
    // }
    pub fn send_words(&self, words: &[&str]) {
        let mut to_send = ["", "", ""];

        // Remplir avec les mots fournis, jusqu'à 3
        for (i, &word) in words.iter().take(3).enumerate() {
            to_send[i] = word;
        }

        // Écrire les mots dans stdin
        if let Ok(mut stdin_lock) = self.stdin.lock() {
            if let Some(ref mut stdin) = *stdin_lock {
                if writeln!(stdin, "{} {} {}", to_send[0], to_send[1], to_send[2]).is_err() {
                    eprintln!("Erreur écriture stdin: Broken Pipe (Python fermé ?)");
                }
            }
        }
    }
}
