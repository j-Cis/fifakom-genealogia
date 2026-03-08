use std::fs;
use std::path::PathBuf;
//use super::data_raw_modele::BazaGeneteki;
use crate::geneteka::data_raw_modele::BazaGeneteki;

/// Przeszukuje folder w poszukiwaniu najnowszego pliku mapa_YYYY_MM_DD.(toml|json)
pub fn znajdz_najnowsza_mape(folder: &str) -> Option<PathBuf> {
    let mut pliki: Vec<PathBuf> = fs::read_dir(folder)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|path| {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            name.starts_with("mapa_") && (name.ends_with(".toml") || name.ends_with(".json"))
        })
        .collect();

    // Domyślne sortowanie alfabetyczne idealnie posortuje daty YYYY_MM_DD rosnąco
    pliki.sort();

    // Zdejmujemy i zwracamy ostatni element (najnowszy)
    pliki.pop()
}

/// Automatycznie znajduje najnowszą mapę w folderze i wczytuje jej zawartość
pub fn laduj_baze(folder: &str) -> Option<BazaGeneteki> {
    let sciezka = znajdz_najnowsza_mape(folder)?;
    println!("Znaleziono najnowszą mapę: {:?}", sciezka);

    let zawartosc = fs::read_to_string(&sciezka).ok()?;

    // Sprawdzamy rozszerzenie, by wiedzieć, jak parsować
    if sciezka.extension().and_then(|e| e.to_str()) == Some("json") {
        // Jeśli będziesz miał JSONy, odkomentuj to i dodaj `serde_json` do Cargo.toml
        // serde_json::from_str(&zawartosc).ok()
        println!("Format JSON nie jest jeszcze w pełni obsługiwany w tym bloku!");
        None
    } else {
        // Deserializacja TOML (Wymaga "toml" w Cargo.toml)
        toml::from_str(&zawartosc).ok()
    }
}
