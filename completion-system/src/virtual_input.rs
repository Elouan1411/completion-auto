use std::collections::HashMap;
use uinput::event::keyboard;
use uinput::Device;

pub fn create_keycode_uinput(is_qwerty: bool) -> HashMap<char, keyboard::Key> {
    // Création de la correspondance entre les caractères et les touches
    let mut keycode_uinput: HashMap<char, keyboard::Key> = [
        ('a', keyboard::Key::A),
        ('b', keyboard::Key::B),
        ('c', keyboard::Key::C),
        ('d', keyboard::Key::D),
        ('e', keyboard::Key::E),
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
        (' ', keyboard::Key::Space),
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
        ];

        for (char, key) in azerty_overrides {
            keycode_uinput.insert(char, key);
        }
    }

    keycode_uinput
}

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

pub fn init_virtual_key() -> Device {
    let device = uinput::default()
        .unwrap()
        .name("my_virtual_keyboard")
        .unwrap()
        .event(uinput::event::Keyboard::All)
        .unwrap()
        .create()
        .unwrap();
    device
}

pub fn write_word(
    word: String,
    device: &mut Device,
    keycode_uinput: &HashMap<char, keyboard::Key>,
) {
    for c in word.chars() {
        press_virtual_key(c.to_ascii_lowercase(), device, keycode_uinput);
    }
}
