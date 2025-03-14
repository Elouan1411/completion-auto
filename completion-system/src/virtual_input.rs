use std::{collections::HashMap, thread, time::Duration};
use uinput::{event::keyboard, Device};

use crate::offset;

/// Creates a keycode map for uinput based on the keyboard layout.
///
/// # Arguments
///
/// * `is_qwerty` - A boolean indicating if the keyboard is QWERTY.
///
/// # Returns
///
/// * `HashMap<char, keyboard::Key>` - A map of characters to uinput keys.
pub fn create_keycode_uinput(is_qwerty: bool) -> HashMap<char, keyboard::Key> {
    // Création de la correspondance entre les caractères et les touches
    let mut keycode_uinput: HashMap<char, keyboard::Key> = [
        ('a', keyboard::Key::A),
        ('à', keyboard::Key::A),
        ('b', keyboard::Key::B),
        ('c', keyboard::Key::C),
        ('d', keyboard::Key::D),
        ('e', keyboard::Key::E),
        ('é', keyboard::Key::E),
        ('è', keyboard::Key::E),
        ('f', keyboard::Key::F),
        ('g', keyboard::Key::G),
        ('h', keyboard::Key::H),
        ('i', keyboard::Key::I),
        ('j', keyboard::Key::J),
        ('k', keyboard::Key::K),
        ('l', keyboard::Key::L),
        ('m', keyboard::Key::M),
        ('n', keyboard::Key::N),
        ('o', keyboard::Key::O),
        ('p', keyboard::Key::P),
        ('q', keyboard::Key::Q),
        ('r', keyboard::Key::R),
        ('s', keyboard::Key::S),
        ('t', keyboard::Key::T),
        ('u', keyboard::Key::U),
        ('ù', keyboard::Key::U),
        ('v', keyboard::Key::V),
        ('w', keyboard::Key::W),
        ('x', keyboard::Key::X),
        ('y', keyboard::Key::Y),
        ('z', keyboard::Key::Z),
        ('0', keyboard::Key::_0),
        ('1', keyboard::Key::_1),
        ('2', keyboard::Key::_2),
        ('3', keyboard::Key::_3),
        ('4', keyboard::Key::_4),
        ('5', keyboard::Key::_5),
        ('6', keyboard::Key::_6),
        ('7', keyboard::Key::_7),
        ('8', keyboard::Key::_8),
        ('9', keyboard::Key::_9),
        (',', keyboard::Key::SemiColon),
        (' ', keyboard::Key::Space),
        ('\'', keyboard::Key::Apostrophe),
        ('\n', keyboard::Key::Enter),
        ('\x08', keyboard::Key::BackSpace),
    ]
    .iter()
    .cloned()
    .collect();

    // Si le clavier est en AZERTY, ajuster le mappage
    if !is_qwerty {
        // Remplacement spécifique pour AZERTY
        let azerty_overrides = [
            ('a', keyboard::Key::Q),
            ('q', keyboard::Key::A),
            ('z', keyboard::Key::W),
            ('w', keyboard::Key::Z),
            ('m', keyboard::Key::SemiColon),
            (',', keyboard::Key::M),
            ('é', keyboard::Key::_2),
            ('è', keyboard::Key::_7),
            ('à', keyboard::Key::_0),
            ('ù', keyboard::Key::Apostrophe),
            ('\'', keyboard::Key::_4),
        ];

        for (char, key) in azerty_overrides {
            keycode_uinput.insert(char, key);
        }
    }

    keycode_uinput
}

/// Presses a virtual key on the device.
///
/// # Arguments
///
/// * `input_char` - The character to press.
/// * `device` - The virtual device.
/// * `keycode_uinput` - A map of characters to uinput keys.
fn press_virtual_key(
    input_char: char,
    device: &mut Device,
    keycode_uinput: &HashMap<char, keyboard::Key>,
) {
    // Vérification si le caractère est valide
    if let Some(key) = keycode_uinput.get(&input_char) {
        // Envoi de la touche spécifiée avec gestion d'erreur
        if let Err(err) = device.click(key) {
            eprintln!("Erreur lors du clic de la touche '{}': {}", input_char, err);
        }

        if let Err(err) = device.synchronize() {
            eprintln!("Erreur lors de la synchronisation : {}", err);
        }
    } else {
        eprintln!("Caractère non mappé : '{}'", input_char);
    }
}

/// Initializes the virtual keyboard device.
///
/// # Returns
///
/// * `Device` - The virtual keyboard device.
pub fn init_virtual_key() -> Device {
    uinput::default()
        .expect("Erreur lors de l'initialisation de uinput")
        .name("my_virtual_keyboard")
        .expect("Erreur lors de l'ajout du nom du clavier")
        .event(uinput::event::Keyboard::All)
        .expect("Erreur lors de l'ajout des événements clavier")
        .create()
        .expect("Erreur lors de la création du périphérique")
}

/// Writes a word using the virtual keyboard.
///
/// # Arguments
///
/// * `word` - The word to write.
/// * `device` - The virtual device.
/// * `keycode_uinput` - A map of characters to uinput keys.
pub fn write_word(
    word: String,
    device: &mut Device,
    keycode_uinput: &HashMap<char, keyboard::Key>,
) {
    for c in word.chars() {
        press_virtual_key(c.to_ascii_lowercase(), device, keycode_uinput);
    }
}

/// Deletes a word using the virtual keyboard.
///
/// # Arguments
///
/// * `size` - The size of the word to delete.
/// * `device` - The virtual device.
/// * `keycode_uinput` - A map of characters to uinput keys.
pub fn delete_word(
    size: usize,
    device: &mut Device,
    keycode_uinput: &HashMap<char, keyboard::Key>,
) {
    for _k in 0..size {
        press_virtual_key('\x08', device, keycode_uinput);
    }
}

/// Changes the active window using the virtual keyboard.
///
/// This function simulates pressing ALT+TAB to switch windows.
pub fn change_window(device: &mut Device) {
    // Appuie sur ALT
    device.press(&keyboard::Key::LeftAlt).unwrap();
    thread::sleep(Duration::from_millis(50)); // Petite pause pour la stabilité

    // Appuie et relâche TAB
    device.press(&keyboard::Key::Tab).unwrap();
    thread::sleep(Duration::from_millis(50));
    device.release(&keyboard::Key::Tab).unwrap();
    thread::sleep(Duration::from_millis(50));

    // Relâche ALT
    device.release(&keyboard::Key::LeftAlt).unwrap();
}

/// Deletes the current word and writes a corrected word using the virtual keyboard.
///
/// # Arguments
///
/// * `word_correction` - The corrected word to write.
/// * `device` - The virtual device.
/// * `keycode_uinput` - A map of characters to uinput keys.
pub fn delete_and_write(
    word_correction: String,
    device: &mut Device,
    keycode_uinput: &HashMap<char, keyboard::Key>,
) {
    change_window(device);
    delete_word(offset::get(), device, keycode_uinput);
    write_word(word_correction, device, keycode_uinput);
}
