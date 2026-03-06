use anyhow::{Context, Result};
use chrono::Local;
use std::fs;
use std::io::Write;
use std::path::Path;

pub async fn pobierz_zakres() -> Result<String> {
    let url = "https://www.google.com/maps/d/kml?mid=1Ig20G_J_1vRHY4aYPmyLj2VqfiDsLkNJ&forcekml=1";

    let dir_path = "./data/geneteka-zakres";
    let dzisiaj = Local::now().format("%Y-%m-%d").to_string();
    let nazwa_pliku = format!("zakres_{}.kml", dzisiaj);
    let pelna_sciezka = format!("{}/{}", dir_path, nazwa_pliku);

    if !Path::new(dir_path).exists() {
        fs::create_dir_all(dir_path).context("Nie udało się utworzyć folderu")?;
    }

    // --- PRZYWRÓCONA BLOKADA POBIERANIA ---
    // Jeśli plik już istnieje, nie męczymy serwera Google,
    // tylko od razu zwracamy czystą ścieżkę do konwersji.
    if Path::new(&pelna_sciezka).exists() {
        println!("[*] Plik KML z dzisiejszą datą już istnieje. Pomijam pobieranie.");
        return Ok(pelna_sciezka);
    }
    // --------------------------------------

    let klient = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .context("Błąd budowania klienta HTTP")?;

    println!("[*] Łączenie z serwerem i pobieranie nowych danych...");

    let odpowiedz = klient
        .get(url)
        .send()
        .await
        .context("Błąd wysyłania zapytania")?;

    if odpowiedz.status().is_success() {
        let bajty = odpowiedz.bytes().await.context("Błąd pobierania bajtów")?;

        if bajty.starts_with(b"<!DOCTYPE html>") || bajty.starts_with(b"<html>") {
            let debug_path = format!("{}/error_log_{}.html", dir_path, dzisiaj);
            fs::write(&debug_path, &bajty)?;
            anyhow::bail!(
                "Otrzymano HTML zamiast KML. Treść błędu zapisano w: {}",
                debug_path
            );
        }

        let mut plik = fs::File::create(&pelna_sciezka).context("Błąd tworzenia pliku")?;
        plik.write_all(&bajty)
            .context("Błąd zapisu danych na dysk")?;

        // Zwracamy TYLKO czystą ścieżkę, bez zbędnych słów "Sukces!"
        Ok(pelna_sciezka)
        // ------------------------
    } else {
        anyhow::bail!("Serwer zwrócił błąd HTTP: {}", odpowiedz.status())
    }
}
