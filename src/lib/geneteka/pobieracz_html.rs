use std::fs;
use std::path::Path;
use chrono::Local;
use anyhow::{Context, Result};
use reqwest::header;

pub async fn pobierz_rejestry() -> Result<String> {
    let url = "https://geneteka.genealodzy.pl/rejestry.php?lang=pol";
    let dir_path = "./data/genealodzy-geneteka/raw";
    let dzisiaj = Local::now().format("%Y-%m-%d").to_string();
    let nazwa_pliku = format!("rejestry_{}.html", dzisiaj);
    let pelna_sciezka = format!("{}/{}", dir_path, nazwa_pliku);

    // 1. Tworzymy folder, jeśli nie istnieje
    if !Path::new(dir_path).exists() {
        fs::create_dir_all(dir_path).context("Nie udało się utworzyć folderu")?;
    }

    // 2. Jeśli plik już tam jest (np. pobrany przez Ciebie ręcznie), pomijamy sieć!
    if Path::new(&pelna_sciezka).exists() {
        println!("[*] Plik HTML z rejestrami z dzisiaj już istnieje na dysku. Pomijam pobieranie.");
        return Ok(pelna_sciezka);
    }

    // 3. BUDUJEMY PEŁNY KAMUFLAŻ PRZEGLĄDARKI
    let mut headers = header::HeaderMap::new();
    
    // Mówimy serwerowi: "Akceptuję cały dokument HTML, tak jak normalna przeglądarka"
    headers.insert(header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert(header::ACCEPT_LANGUAGE, "pl-PL,pl;q=0.9,en-US;q=0.8,en;q=0.7".parse().unwrap());
    // Udajemy, że weszliśmy z głównej strony Geneteki
    headers.insert(header::REFERER, "https://geneteka.genealodzy.pl/".parse().unwrap());
    headers.insert(header::CONNECTION, "keep-alive".parse().unwrap());
    headers.insert(header::UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());

    // Główny identyfikator: Pełny, prawdziwy Google Chrome na Windows 10/11
    let klient = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .default_headers(headers)
        // Ustawiamy długi timeout, bo wygenerowanie tego wielkiego pliku przez serwer PTG trochę trwa
        .timeout(std::time::Duration::from_secs(60)) 
        .build()
        .context("Błąd budowania klienta HTTP")?;

    println!("[*] Łączenie z Geneteką w trybie kamuflażu (Chrome) i pobieranie całego pliku...");

    // 4. WYSYŁAMY ZAPYTANIE
    let odpowiedz = klient.get(url)
        .send()
        .await
        .context("Błąd wysyłania zapytania do Geneteki")?;

    if odpowiedz.status().is_success() {
        // Pobieramy CAŁY tekst odpowiedzi
        let tekst_html = odpowiedz.text().await.context("Błąd odczytu tekstu HTML")?;
        
        // 5. ZABEZPIECZENIE: Sprawdzamy, czy serwer nie rzucił błędem bazy zamiast wysłać tabelę!
        if tekst_html.contains("Connect to database error") {
            anyhow::bail!("Serwer Geneteki jest przeciążony i wyrzucił błąd bazy danych (Connect to database error). Spróbuj ponownie później lub użyj pliku pobranego ręcznie.");
        }

        // 6. Zapisujemy ten potężny plik na dysk
        fs::write(&pelna_sciezka, &tekst_html).context("Błąd zapisu pliku HTML")?;
        
        Ok(pelna_sciezka)
    } else {
        anyhow::bail!("Serwer odrzucił połączenie. Kod błędu HTTP: {}", odpowiedz.status())
    }
}