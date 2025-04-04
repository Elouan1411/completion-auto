use std::sync::atomic::{AtomicUsize, Ordering};

pub static OFFSET: AtomicUsize = AtomicUsize::new(0);

/// Increases the offset by 1.
pub fn add() {
    OFFSET.fetch_add(1, Ordering::Relaxed);
}

/// Decreases the offset by 1 if it is greater than 0.
pub fn subb() {
    let value = OFFSET.load(Ordering::Relaxed);
    if value > 0 {
        OFFSET.fetch_sub(1, Ordering::Relaxed);
    }
}

/// Resets the offset to 0.
pub fn reset() {
    OFFSET.store(0, Ordering::Relaxed);
}

/// Gets the current value of the offset.
///
/// # Returns
///
/// * `usize` - The current offset value.
pub fn get() -> usize {
    OFFSET.load(Ordering::Relaxed)
}

/// Manages the word based on the input letter.
///
/// # Arguments
///
/// * `letter` - The input letter.
/// * `word` - The word being managed.
pub fn manage_word(letter: &mut String, word: &mut String) {
    // Gestion du mot
    if letter == "backspace" {
        if get() >= 1 {
            word.remove(get() - 1);
            subb();
        }
    } else if letter == "left" {
        if get() == 0 {
            word.clear();
        } else {
            subb();
        }
    } else if letter == "right" {
        if get() < word.len() {
            add();
        } else {
            word.clear();
            reset();
        }
    }
    // Vérifier si la lettre contient un seul caractère et si ce caractère est alphabétique
    else if letter.chars().count() == 1 {
        if let Some(first_char) = letter.chars().next() {
            if first_char.is_alphabetic() || "éèàùç'".contains(first_char) {
                word.insert(get(), first_char);
                add();
            } else {
                // Si ce n'est pas une lettre, on vide le mot
                word.clear();
                reset();
            }
        }
    } else {
        word.clear();
        reset();
    }
}
