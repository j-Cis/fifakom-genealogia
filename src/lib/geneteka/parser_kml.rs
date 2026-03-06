use anyhow::{Context, Result};
use roxmltree::Document;
use std::fs;

// Importujemy nasze modele i parser zakresu z sąsiednich plików
use super::modele::{Miejsce, Rekord, Roczniki};
use super::parser_rocznikow::rozkoduj_lata;

/// Główna funkcja czytająca plik KML i zapisująca wynik do JSON
pub fn parsuj_do_json(sciezka_kml: &str, sciezka_json: &str) -> Result<()> {
    println!("[*] Wczytywanie pliku KML do pamięci: {}", sciezka_kml);
    let xml_tekst = fs::read_to_string(sciezka_kml).context("Błąd odczytu pliku KML")?;

    println!("[*] Analiza drzewa XML...");
    let doc = Document::parse(&xml_tekst).context("Błąd parsowania struktury XML")?;

    let mut wyniki: Vec<Rekord> = Vec::new();
    let mut lp = 0;

    // Przeszukujemy całe drzewo XML w poszukiwaniu tagów <Placemark>
    for node in doc
        .descendants()
        .filter(|n| n.tag_name().name() == "Placemark")
    {
        let nazwa_parafii = znajdz_tekst_dziecka(&node, "name").unwrap_or_default();

        let mut lon_lat: (f64, f64) = (0.0, 0.0);
        if let Some(point) = node.children().find(|n| n.tag_name().name() == "Point")
            && let Some(coords) = znajdz_tekst_dziecka(&point, "coordinates")
        {
            let czesci: Vec<&str> = coords.split(',').collect();
            if czesci.len() >= 2 {
                lon_lat.0 = czesci[0].trim().parse().unwrap_or(0.0); // Długość
                lon_lat.1 = czesci[1].trim().parse().unwrap_or(0.0); // Szerokość
            }
        }

        let mut teren = String::new();
        let mut zakres_tekst = String::new();
        let mut link = String::new();

        // Wyciąganie danych z <ExtendedData>
        if let Some(ext_data) = node
            .children()
            .find(|n| n.tag_name().name() == "ExtendedData")
        {
            for data_node in ext_data
                .children()
                .filter(|n| n.tag_name().name() == "Data")
            {
                let atrybut_nazwa = data_node.attribute("name").unwrap_or("");
                let wartosc = znajdz_tekst_dziecka(&data_node, "value").unwrap_or_default();

                match atrybut_nazwa {
                    "Obszar/województwo" => teren = wartosc,
                    "Zakres" => zakres_tekst = wartosc,
                    "Link do Geneteki" => link = wartosc,
                    _ => {}
                }
            }
        }

        // --- UWAGA: ŁATKA Z TWOJEGO SKRYPTU JS (Aleksandrowo) ---
        if (lon_lat.0 - 20.8017713).abs() < 0.0001 && (lon_lat.1 - 52.5337256).abs() < 0.0001 {
            zakres_tekst = "U 1846,61-62".to_string();
            link = "https://geneteka.genealodzy.pl/index.php?op=gt&w=07mz&rid=9687".to_string();
        }
        // --------------------------------------------------------

        // Parsowanie linku z użyciem prostej funkcji pomocniczej
        let kod_rid = wyciagnij_parametr(&link, "rid").unwrap_or_default();
        let kod_w = wyciagnij_parametr(&link, "w").unwrap_or_default();

        let mut zakres_dziedzin = Roczniki::nowy();

        for czesc in zakres_tekst.split(';') {
            let czesc = czesc.trim();
            if czesc.is_empty() {
                continue;
            }
            let dziedzina = czesc.chars().next().unwrap_or(' ').to_ascii_uppercase();
            let reszta = czesc[1..].trim();
            let lata = rozkoduj_lata(reszta);
            match dziedzina {
                'U' => zakres_dziedzin.u.extend(lata),
                'M' => zakres_dziedzin.m.extend(lata),
                'Z' => zakres_dziedzin.z.extend(lata),
                _ => {}
            }
        }

        zakres_dziedzin.u.sort_unstable();
        zakres_dziedzin.u.dedup();
        zakres_dziedzin.m.sort_unstable();
        zakres_dziedzin.m.dedup();
        zakres_dziedzin.z.sort_unstable();
        zakres_dziedzin.z.dedup();

        // Składanie "miejsca" z nazwy i kodu
        let mut parafia_vec = vec![nazwa_parafii];
        if !kod_rid.is_empty() {
            parafia_vec.push(kod_rid);
        }

        let mut obszar_vec = vec![teren];
        if !kod_w.is_empty() {
            obszar_vec.push(kod_w);
        }

        // Nasza nowa, zgrabna struktura
        let rekord = Rekord {
            lp,
            miejsce: Miejsce {
                lonlat: [lon_lat.0, lon_lat.1],
                obszar: obszar_vec,
                parafia: parafia_vec,
            },
            roczniki: zakres_dziedzin, // <-- Używamy nowej nazwy pola "roczniki"
        };

        wyniki.push(rekord);
        lp += 1;
    }

    println!("[+] Przetworzono {} parafii z pliku XML.", wyniki.len());

    // --- 1. Zapis do hybrydowego JSON ---
    let json_linie: Vec<String> = wyniki
        .iter()
        .map(|r| serde_json::to_string(r).unwrap_or_default())
        .collect();

    let json_dane = format!("[\n  {}\n]", json_linie.join(",\n  "));
    fs::write(sciezka_json, json_dane).context("Błąd zapisu pliku JSON")?;
    println!("[+] Zapisano idealny format JSON: {}", sciezka_json);

    // --- 2. Zapis do zwartego TOML ---
    let sciezka_toml = sciezka_json.replace(".json", ".toml");
    let mut toml_dane = String::new();

    for r in &wyniki {
        // Zmieniliśmy r.zakres na r.roczniki!
        let u_str = serde_json::to_string(&r.roczniki.u).unwrap_or_default();
        let m_str = serde_json::to_string(&r.roczniki.m).unwrap_or_default();
        let z_str = serde_json::to_string(&r.roczniki.z).unwrap_or_default();
        let parafia_str = serde_json::to_string(&r.miejsce.parafia).unwrap_or_default();
        let obszar_str = serde_json::to_string(&r.miejsce.obszar).unwrap_or_default();

        // Zmieniliśmy na sztywno wpisane słowo "zakres =" na "roczniki ="
        toml_dane.push_str(&format!(
            "[[rekord]]\nlp = {}\nmiejsce = {{ parafia = {}, obszar = {}, lonlat = [{}, {}] }}\nroczniki = {{ u = {}, m = {}, z = {} }}\n\n",
            r.lp, parafia_str, obszar_str, r.miejsce.lonlat[0], r.miejsce.lonlat[1], u_str, m_str, z_str
        ));
    }

    fs::write(&sciezka_toml, toml_dane).context("Błąd zapisu pliku TOML")?;
    println!("[+] Zapisano super-zwarty plik TOML: {}", sciezka_toml);

    Ok(())
}

// --- FUNKCJE POMOCNICZE ---

/// Szybkie szukanie tekstu wewnątrz danego tagu
fn znajdz_tekst_dziecka(node: &roxmltree::Node, nazwa_tagu: &str) -> Option<String> {
    node.children()
        .find(|n| n.tag_name().name() == nazwa_tagu)
        .and_then(|n| n.text())
        .map(|s| s.to_string())
}

/// Zastępuje `g_link` z JS. Szuka w linku wartości po `klucz=`
fn wyciagnij_parametr(url: &str, klucz: &str) -> Option<String> {
    let fragmenty: Vec<&str> = url.split(&['?', '&'][..]).collect();
    let szukany_prefix = format!("{}=", klucz);

    for f in fragmenty {
        if let Some(wartosc) = f.strip_prefix(&szukany_prefix) {
            return Some(wartosc.to_string());
        }
    }
    None
}
