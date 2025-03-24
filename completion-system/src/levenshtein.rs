use std::fs::File;
use std::io::{self, BufRead};
// use std::path::Path;

/// Calculates the Levenshtein distance between two strings.
///
/// # Arguments
///
/// * `a` - The first string.
/// * `b` - The second string.
///
/// # Returns
///
/// * `usize` - The Levenshtein distance.
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 0..=n {
        for j in 0..=m {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else if a.chars().nth(i - 1) == b.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
            }
        }
    }

    dp[n][m]
}

/// Calculates the optimized Levenshtein distance between two strings.
///
/// # Arguments
///
/// * `a` - The first string.
/// * `b` - The second string.
///
/// # Returns
///
/// * `usize` - The optimized Levenshtein distance.
pub fn optimized_levenshtein(a: &str, b: &str) -> usize {
    let n = a.len();
    let m = b.len();
    let mut prev: Vec<usize> = (0..=m).collect();
    let mut curr = vec![0; m + 1];

    for i in 1..=n {
        curr[0] = i;
        for j in 1..=m {
            let insert = curr[j - 1] + 1;
            let delete = prev[j] + 1;
            let substitute = prev[j - 1]
                + if a.chars().nth(i - 1) == b.chars().nth(j - 1) {
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

/// Provides suggestions for word completion based on the Levenshtein distance.
///
/// # Arguments
///
/// * `partiel` - The partial word.
/// * `dictionnaire` - The dictionary of words.
/// * `seuil` - The maximum Levenshtein distance for suggestions.
/// * `max_suggestions` - The maximum number of suggestions.
///
/// # Returns
///
/// * `Vec<String>` - A list of suggested words.
pub fn suggestions_completion(
    partiel: &str,
    dictionnaire: &[String],
    seuil: usize,
    max_suggestions: usize,
) -> Vec<String> {
    let mut distances: Vec<(String, usize)> = Vec::new();

    for mot in dictionnaire {
        if (mot.len() as isize - partiel.len() as isize).abs() as usize <= seuil {
            let dist = optimized_levenshtein(partiel, mot);
            if dist <= seuil {
                distances.push((mot.clone(), dist));
            }
        }
    }

    distances.sort_by_key(|&(_, dist)| dist);
    distances
        .into_iter()
        .take(max_suggestions)
        .map(|(mot, _)| mot)
        .collect()
}

/// Reads the dictionary from a file.
///
/// # Arguments
///
/// * `file_path` - The path to the dictionary file.
///
/// # Returns
///
/// * `Vec<String>` - A list of words from the dictionary.
pub fn read_dictionary(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

/// Provides suggestions for a given word using a dictionary file.
///
/// # Arguments
///
/// * `word` - The word to get suggestions for.
/// * `dictionary_path` - The path to the dictionary file.
///
/// # Returns
///
/// * `io::Result<Vec<String>>` - A list of suggested words.
pub fn get_suggestions(word: &str, dictionary_path: &str) -> io::Result<Vec<String>> {
    let dictionary = read_dictionary(dictionary_path)?;
    let suggestions = suggestions_completion(word, &dictionary, 3, 3);
    Ok(suggestions)
}
