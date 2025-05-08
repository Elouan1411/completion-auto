use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn get_suggestions(word: &str, dictionary_contents: &str) -> Vec<String> {
    let word_lower = word.to_lowercase();
    let mut heap = BinaryHeap::with_capacity(4); // Petite optimisation de mémoire

    for line in dictionary_contents.lines() {
        let mut parts = line.split(',');
        let mot = parts.next().unwrap_or("").trim().to_lowercase();
        let freq = parts
            .next()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0);

        if mot.starts_with(&word_lower) {
            heap.push((Reverse(freq), mot));

            if heap.len() > 3 {
                heap.pop();
            }
        }
    }

    heap.into_sorted_vec()
        .into_iter()
        .rev() // Plus besoin de reverse après
        .map(|(_, mot)| mot)
        .collect()
}
