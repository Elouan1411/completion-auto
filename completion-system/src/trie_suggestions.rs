use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn optimized_levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let n = a_chars.len();
    let m = b_chars.len();

    let mut prev: Vec<usize> = (0..=m).collect();
    let mut curr = vec![0; m + 1];

    for i in 1..=n {
        curr[0] = i;
        for j in 1..=m {
            let insert = curr[j - 1] + 1;
            let delete = prev[j] + 1;
            let substitute = prev[j - 1]
                + if a_chars[i - 1] == b_chars[j - 1] {
                    0
                } else {
                    1
                };
            curr[j] = insert.min(delete).min(substitute);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[m]
}

#[derive(Debug, Eq, PartialEq)]
struct Suggestion {
    distance: usize,
    is_prefix: bool,
    frequency: usize,
    word: String,
}

impl Ord for Suggestion {
    // définir l'ordre des elements dans le BinaryHeap
    // tri par distance puis is_prefixe puis frequency
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (
            self.distance,
            Reverse(self.is_prefix),
            Reverse(self.frequency),
        )
            .cmp(&(
                other.distance,
                Reverse(other.is_prefix),
                Reverse(other.frequency),
            ))
    }
}

impl PartialOrd for Suggestion {
    // eviter les bugs
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn get_suggestions(word: &str, dictionary_contents: &str) -> Vec<String> {
    const MAX_DISTANCE: usize = 2;
    let mut heap = BinaryHeap::with_capacity(4);

    for line in dictionary_contents.lines() {
        let mut parts = line.split(',');
        let mot = parts.next().unwrap_or("").trim();
        let freq = parts
            .next()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0);

        let is_prefix = mot.starts_with(word);
        let distance = optimized_levenshtein(word, mot);

        if distance > MAX_DISTANCE && !is_prefix {
            // on ignore pas le mot si la distance est trop
            // grande mais que c'est le même préfixe
            continue;
        }

        heap.push(Suggestion {
            distance,
            is_prefix: is_prefix && distance == 0, // Bonus pour le préfixe exact
            frequency: freq,
            word: mot.to_string(),
        });

        if heap.len() > 3 {
            heap.pop();
        }
    }

    let mut suggestions = Vec::with_capacity(3);
    while let Some(suggestion) = heap.pop() {
        suggestions.push(suggestion.word);
    }
    suggestions.reverse();
    suggestions
}
