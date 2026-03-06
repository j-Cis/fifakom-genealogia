// src/logic/morphology.rs
use itertools::Itertools;
use regex::Regex;
use std::sync::LazyLock;

// Wyrażenie zostanie skompilowane tylko raz, przy pierwszym użyciu
static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\(([^)]+)\)|[^()\s]+").unwrap());

/// Funkcja generująca tablicę wyrazów według wzoru morfologicznego
pub fn generate_morphology(pattern: &str) -> Vec<String> {
    // Regex: dopasuj grupy w nawiasach lub wszystko, co nie jest nawiasem (ciągi dołączane bez spacji)
    //let re = Regex::new(r"\(([^)]+)\)|[^()\s]+").unwrap();
    let mut parts: Vec<Vec<String>> = vec![];

    // 1. Rozbijanie wzoru na grupy i fragmenty
    for cap in RE.captures_iter(pattern) {
        if let Some(group) = cap.get(1) {
            // grupy w nawiasach → rozdzielone |, usuń spacje
            let options: Vec<String> = group
                .as_str()
                .split('|')
                .map(|s| s.trim().to_string())
                .collect();
            parts.push(options);
        } else {
            // fragmenty poza nawiasami → pojedynczy string
            parts.push(vec![cap[0].to_string()]);
        }
    }

    // 2. Kombinatoryka (Produkt kartezjański)
    //// Kombinatoryka: łączymy wszystkie części w jeden string
    //let mut results = vec!["".to_string()];
    //for part in parts {
    //	let mut new_results = vec![];
    //	for prefix in &results {
    //		for option in &part {
    //			new_results.push(format!("{}{}", prefix, option));
    //		}
    //	}
    //	results = new_results;
    //}
    //
    //results

    parts
        .into_iter()
        .multi_cartesian_product()
        .map(|kombinacja| kombinacja.join(""))
        .collect()
}
