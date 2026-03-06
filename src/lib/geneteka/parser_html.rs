use scraper::{Html, Selector};
use std::collections::HashMap;
use std::fs;
use anyhow::{Context, Result};

use super::modele::{Miejsce, Rekord, Roczniki};
use super::parser_rocznikow::rozkoduj_lata;

/// Główna funkcja parsująca pobrany HTML do JSON i TOML
pub fn parsuj_html(sciezka_html: &str, sciezka_json: &str) -> Result<()> {
    println!("[*] Wczytywanie pliku HTML do pamięci: {}", sciezka_html);
    let html_tekst = fs::read_to_string(sciezka_html).context("Błąd odczytu pliku HTML")?;

    println!("[*] Analiza drzewa HTML (to może chwilę potrwać)...");
    let document = Html::parse_document(&html_tekst);

    // Selektory CSS - dokładnie tak, jak działa przeglądarka internetowa
    let tr_selector = Selector::parse("table[border=\"1\"] tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    // Kluczem w mapie będzie: "kodWojewództwa_NazwaParafii" 
    // (żeby sprytnie połączyć U, M, Z z różnych linków w jeden rekord)
    let mut mapa_parafii: HashMap<String, Rekord> = HashMap::new();

    for tr in document.select(&tr_selector) {
        let tds: Vec<_> = tr.select(&td_selector).collect();
        
        // Prawidłowy wiersz z województwem ma co najmniej 4 kolumny
        if tds.len() >= 4 {
            let teren = tds[0].text().collect::<String>().trim().to_string();
            
            if teren == "Tereny" || teren.is_empty() { continue; }

            // Przeszukujemy 4. kolumnę w poszukiwaniu parafii (tagi <a>)
            for a in tds[3].select(&a_selector) {
                let href = a.value().attr("href").unwrap_or("");
                let title = a.value().attr("title").unwrap_or(""); // Tu kryją się roczniki!
                let klasa = a.value().attr("class").unwrap_or(""); // B, S, D
                let nazwa_parafii = a.text().collect::<String>().trim().to_string();

                let kod_rid = wyciagnij_parametr(href, "rid").unwrap_or_default();
                let kod_w = wyciagnij_parametr(href, "w").unwrap_or_default();

                if kod_w.is_empty() || nazwa_parafii.is_empty() { continue; }

                let klucz = format!("{}_{}", kod_w, nazwa_parafii);
                let lata = rozkoduj_lata(title);

                let rekord = mapa_parafii.entry(klucz).or_insert_with(|| {
                    Rekord {
                        lp: 0,
                        miejsce: Miejsce {
                            lonlat: [0.0, 0.0], // Z HTML nie mamy mapy
                            obszar: vec![teren.clone(), kod_w.clone()],
                            parafia: vec![nazwa_parafii.clone()],
                        },
                        roczniki: Roczniki::nowy(),
                    }
                });

                // Jeśli parafia ma nowy RID (np. inny dla U, a inny dla Z), dopisujemy go
                if !kod_rid.is_empty() && !rekord.miejsce.parafia.contains(&kod_rid) {
                    rekord.miejsce.parafia.push(kod_rid);
                }

                // Rozrzucamy lata do pojemników na podstawie klasy linku (B=U, S=M, D=Z)
                match klasa {
                    "B" => rekord.roczniki.u.extend(lata), 
                    "S" => rekord.roczniki.m.extend(lata), 
                    "D" => rekord.roczniki.z.extend(lata), 
                    _ => {}
                }
            }
        }
    }

    let mut wyniki: Vec<Rekord> = mapa_parafii.into_values().collect();
    
    // Sortujemy elegancko: najpierw kod województwa (np. 02kp), potem nazwa parafii
    wyniki.sort_by(|a, b| {
        a.miejsce.obszar.get(1).unwrap_or(&String::new()).cmp(b.miejsce.obszar.get(1).unwrap_or(&String::new()))
            .then_with(|| a.miejsce.parafia[0].cmp(&b.miejsce.parafia[0]))
    });

    for (i, rekord) in wyniki.iter_mut().enumerate() {
        rekord.lp = i;
        rekord.roczniki.u.sort_unstable(); rekord.roczniki.u.dedup();
        rekord.roczniki.m.sort_unstable(); rekord.roczniki.m.dedup();
        rekord.roczniki.z.sort_unstable(); rekord.roczniki.z.dedup();
    }

    println!("[+] Przetworzono {} unikalnych parafii z tabeli HTML.", wyniki.len());

    // --- Zapis do JSON ---
    let json_linie: Vec<String> = wyniki.iter()
        .map(|r| serde_json::to_string(r).unwrap_or_default())
        .collect();
    let json_dane = format!("[\n  {}\n]", json_linie.join(",\n  "));
    fs::write(sciezka_json, json_dane).context("Błąd zapisu pliku JSON")?;
    println!("[+] Zapisano idealny format JSON: {}", sciezka_json);

    // --- Zapis do TOML ---
    let sciezka_toml = sciezka_json.replace(".json", ".toml");
    let mut toml_dane = String::new();
    for r in &wyniki {
        let u_str = serde_json::to_string(&r.roczniki.u).unwrap_or_default();
        let m_str = serde_json::to_string(&r.roczniki.m).unwrap_or_default();
        let z_str = serde_json::to_string(&r.roczniki.z).unwrap_or_default();
        let parafia_str = serde_json::to_string(&r.miejsce.parafia).unwrap_or_default();
        let obszar_str = serde_json::to_string(&r.miejsce.obszar).unwrap_or_default();
        
        toml_dane.push_str(&format!(
            "[[rekord]]\nlp = {}\nmiejsce = {{ parafia = {}, obszar = {}, lonlat = [{}, {}] }}\nroczniki = {{ u = {}, m = {}, z = {} }}\n\n",
            r.lp, parafia_str, obszar_str, r.miejsce.lonlat[0], r.miejsce.lonlat[1], u_str, m_str, z_str
        ));
    }
    fs::write(&sciezka_toml, toml_dane).context("Błąd zapisu pliku TOML")?;
    println!("[+] Zapisano super-zwarty plik TOML: {}", sciezka_toml);

    Ok(())
}

fn wyciagnij_parametr(url: &str, klucz: &str) -> Option<String> {
    let czysty_url = url.replace("&amp;", "&");
    let fragmenty: Vec<&str> = czysty_url.split(&['?', '&'][..]).collect();
    let szukany_prefix = format!("{}=", klucz);
    
    for f in fragmenty {
        if let Some(wartosc) = f.strip_prefix(&szukany_prefix) {
            return Some(wartosc.to_string());
        }
    }
    None
}