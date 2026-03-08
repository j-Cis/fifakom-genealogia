# RAPORT KODU: doc/code-rust-lib.md

## Plik-RustLibPub_01_01: `src/lib/atlas/generator.rs`

```rust
use super::modele::{MapProcessedData, MapProjection, NormalizedPoint};
use super::projekcje::{proj_dynamiczna, proj_plate_carree};
use crate::geneteka::data_raw_modele::Rekord;
use crate::pliki::path_data_naturalearth as paths;
use geojson::{GeoJson, Value};
use std::fs;

pub fn generate_map_data(records: &[Rekord], projection: MapProjection) -> MapProcessedData {
    let (min_lon, max_lon, min_lat, max_lat) = match projection {
        MapProjection::ProjWgs84 => proj_plate_carree::get_bounds(),
        MapProjection::ProjDynamic { margin } => proj_dynamiczna::get_bounds(records, margin),
    };

    let lon_range = max_lon - min_lon;
    let lat_range = max_lat - min_lat;

    let mut coastlines = Vec::new();

    // FAST-FALL: Używamy Twoich dokładnych nazw z rozszerzeniem .geojson
    let sciezki_geo = [
        paths::PATH_COASTLINE_50M,
        paths::PATH_GLACIERS_110M,
        paths::PATH_LAKES_50M,
        paths::PATH_RIVERS_LAKE_CENTER_50M,
        paths::PATH_RAMKA,
    ];

    for sciezka in sciezki_geo {
        // .expect spowoduje CRASH i poda ścieżkę, jeśli pliku nie ma pod tym adresem
        let data = fs::read_to_string(sciezka)
            .unwrap_or_else(|_| panic!("KRYTYCZNY BŁĄD: Nie znaleziono pliku: {}", sciezka));

        let res: GeoJson = data.parse().unwrap_or_else(|_| {
            panic!(
                "KRYTYCZNY BŁĄD: Plik {} nie jest poprawnym GeoJSONem",
                sciezka
            )
        });

        wyciagnij_linie(&res, &mut coastlines, min_lon, max_lon, min_lat, max_lat);
    }

    // Diagnostyka w konsoli
    println!(
        ">>> GENERATOR: Wczytano {} segmentów mapy bazowej",
        coastlines.len()
    );

    let mut points = Vec::new();
    for rek in records {
        if rek.miejsce.lonlat.len() == 2 {
            let lon = rek.miejsce.lonlat[0];
            let lat = rek.miejsce.lonlat[1];
            let name = rek.miejsce.parafia.first().cloned().unwrap_or_default();

            let x_norm = (lon - min_lon) / lon_range;
            let y_norm = (lat - min_lat) / lat_range;

            if (0.0..=1.0).contains(&x_norm) && (0.0..=1.0).contains(&y_norm) {
                points.push(NormalizedPoint {
                    x: x_norm as f32,
                    y: (1.0 - y_norm) as f32,
                    name,
                });
            }
        }
    }

    MapProcessedData {
        points,
        coastlines,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    }
}

fn wyciagnij_linie(
    gj: &GeoJson,
    out: &mut Vec<Vec<(f32, f32)>>,
    min_lon: f64,
    max_lon: f64,
    min_lat: f64,
    max_lat: f64,
) {
    let lon_range = max_lon - min_lon;
    let lat_range = max_lat - min_lat;

    if let GeoJson::FeatureCollection(fc) = gj {
        for feature in &fc.features {
            if let Some(geom) = &feature.geometry {
                match &geom.value {
                    Value::LineString(ls) => {
                        let coords: Vec<(f32, f32)> = ls
                            .iter()
                            .map(|p| {
                                let x = (p[0] - min_lon) / lon_range;
                                let y = (max_lat - p[1]) / lat_range;
                                (x as f32, y as f32)
                            })
                            .collect();
                        out.push(coords);
                    }
                    Value::MultiLineString(mls) => {
                        for ls in mls {
                            let coords: Vec<(f32, f32)> = ls
                                .iter()
                                .map(|p| {
                                    let x = (p[0] - min_lon) / lon_range;
                                    let y = (max_lat - p[1]) / lat_range;
                                    (x as f32, y as f32)
                                })
                                .collect();
                            out.push(coords);
                        }
                    }
                    Value::Polygon(p) => {
                        for ring in p {
                            let coords: Vec<(f32, f32)> = ring
                                .iter()
                                .map(|p| {
                                    let x = (p[0] - min_lon) / lon_range;
                                    let y = (max_lat - p[1]) / lat_range;
                                    (x as f32, y as f32)
                                })
                                .collect();
                            out.push(coords);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

```

## Plik-RustLibMod_01_00: `src/lib/atlas/mod.rs`

```rust
pub mod generator;
pub mod modele;
pub mod projekcje;
pub mod renderer;

// Dzięki temu w main.rs po prostu wpiszesz `use fifak_lib::atlas::generate_map_data;`
pub use generator::generate_map_data;
pub use modele::{MapProcessedData, MapProjection, NormalizedPoint};
pub use renderer::render_frame;

```

## Plik-RustLibPub_01_02: `src/lib/atlas/modele.rs`

```rust
#[derive(Debug, Clone, Copy)]
pub enum MapProjection {
    ProjDynamic { margin: f64 },
    ProjWgs84,
    // W przyszłości:
    // Dymaxion,
    // AuthaGraph,
}

pub struct NormalizedPoint {
    pub x: f32,
    pub y: f32,
    pub name: String,
}

pub struct MapProcessedData {
    pub points: Vec<NormalizedPoint>,
    pub coastlines: Vec<Vec<(f32, f32)>>,
    pub min_lon: f64,
    pub max_lon: f64,
    pub min_lat: f64,
    pub max_lat: f64,
}

```

## Plik-RustLibMod_01_03_00: `src/lib/atlas/projekcje/mod.rs`

```rust
pub mod proj_dynamiczna;
pub mod proj_plate_carree;

```

## Plik-RustLibPub_01_03_01: `src/lib/atlas/projekcje/proj_dynamiczna.rs`

```rust
use crate::geneteka::data_raw_modele::Rekord;

pub fn get_bounds(records: &[Rekord], margin: f64) -> (f64, f64, f64, f64) {
    let mut m_lon = f64::MAX;
    let mut mx_lon = f64::MIN;
    let mut m_lat = f64::MAX;
    let mut mx_lat = f64::MIN;

    for rek in records {
        if rek.miejsce.lonlat.len() == 2 {
            let lon = rek.miejsce.lonlat[0];
            let lat = rek.miejsce.lonlat[1];
            if lon < m_lon {
                m_lon = lon;
            }
            if lon > mx_lon {
                mx_lon = lon;
            }
            if lat < m_lat {
                m_lat = lat;
            }
            if lat > mx_lat {
                mx_lat = lat;
            }
        }
    }

    let lon_range = mx_lon - m_lon;
    let lat_range = mx_lat - m_lat;

    (
        m_lon - lon_range * margin,
        mx_lon + lon_range * margin,
        m_lat - lat_range * margin,
        mx_lat + lat_range * margin,
    )
}

```

## Plik-RustLibPub_01_03_02: `src/lib/atlas/projekcje/proj_plate_carree.rs`

```rust
pub fn get_bounds() -> (f64, f64, f64, f64) {
    // min_lon, max_lon, min_lat, max_lat
    (-180.0, 180.0, -90.0, 90.0)
}

// Docelowo tu dojdzie np. pub fn project(lon: f64, lat: f64) -> (f64, f64)

```

## Plik-RustLibPub_01_04: `src/lib/atlas/renderer.rs`

```rust
use super::modele::MapProcessedData;
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

pub fn render_frame(
    width: u32,
    height: u32,
    map_data: &MapProcessedData,
    offset_x: f32,
    offset_y: f32,
    zoom: f32,
    rotation_deg: f32,
) -> Image {
    if width == 0 || height == 0 {
        return Image::default();
    }

    let mut pixmap = Pixmap::new(width, height).unwrap();
    // Tło - ustawiam na bardzo ciemny szary, żeby lądy były widoczne
    pixmap.fill(Color::from_rgba8(30, 30, 35, 255));

    let rot_rad = rotation_deg.to_radians();
    let cos_a = rot_rad.cos();
    let sin_a = rot_rad.sin();

    let world_w = width as f32 * zoom;
    let world_h = height as f32 * zoom;
    let pivot_x = world_w / 2.0;
    let pivot_y = world_h / 2.0;

    // --- 1. RYSOWANIE LINII BRZEGOWYCH (MAPA BAZOWA) ---
    let mut paint_map = Paint::default();
    // Ustawiam JASKRAWY ZIELONY na moment testu, żebyś widział czy w ogóle coś rysuje
    paint_map.set_color_rgba8(0, 255, 100, 255);
    paint_map.anti_alias = true;
    let stroke = Stroke {
        width: 1.0,
        ..Default::default()
    };

    for linia in &map_data.coastlines {
        if linia.is_empty() {
            continue;
        }
        let mut pb = PathBuilder::new();

        for (i, p) in linia.iter().enumerate() {
            // MATEMATYKA IDENTYCZNA JAK DLA KROPEK
            let local_x = p.0 * world_w;
            let local_y = p.1 * world_h;
            let dx = local_x - pivot_x;
            let dy = local_y - pivot_y;
            let rot_x = dx * cos_a - dy * sin_a;
            let rot_y = dx * sin_a + dy * cos_a;
            let sx = pivot_x + rot_x + offset_x;
            let sy = pivot_y + rot_y + offset_y;

            if i == 0 {
                pb.move_to(sx, sy);
            } else {
                pb.line_to(sx, sy);
            }
        }

        if let Some(path) = pb.finish() {
            pixmap.stroke_path(&path, &paint_map, &stroke, Transform::identity(), None);
        }
    }

    // --- 2. RYSOWANIE PUNKTÓW (GENETEKA) ---
    let mut paint_pt = Paint::default();
    paint_pt.set_color_rgba8(255, 0, 85, 255);
    paint_pt.anti_alias = true;

    for pt in &map_data.points {
        let local_x = pt.x * world_w;
        let local_y = pt.y * world_h;
        let dx = local_x - pivot_x;
        let dy = local_y - pivot_y;
        let rot_x = dx * cos_a - dy * sin_a;
        let rot_y = dx * sin_a + dy * cos_a;
        let sx = pivot_x + rot_x + offset_x;
        let sy = pivot_y + rot_y + offset_y;

        if sx >= -5.0
            && sx <= width as f32 + 5.0
            && sy >= -5.0
            && sy <= height as f32 + 5.0
            && let Some(rect) = Rect::from_xywh(sx - 2.0, sy - 2.0, 4.0, 4.0)
        {
            pixmap.fill_rect(rect, &paint_pt, Transform::identity(), None);
        }
    }

    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(pixmap.data(), width, height);
    Image::from_rgba8(buffer)
}

```

## Plik-RustLibPub_02_01: `src/lib/cli/menu.rs`

```rust
use crate::logic::format::OutputFormat;
use crate::logic::format::format_result;
use crate::logic::morphology::generate_morphology;
use inquire::{Select, Text};

pub fn run_menu() {
    let mut pattern = "(abc | def) ijk (1 | 2)".to_string();
    let mut output_format = OutputFormat::Lista; // Korzystamy z enuma!
    let mut running = true;

    // Przeniesione przed pętlę i użyte jako tablica (slice), nie wektor:
    let menu_options = [
        "Generuj",
        "Zmień wzór morfologiczny",
        "Zmień formę rezultatu",
        "Zmień sortowanie/grupowanie",
        "Ustawienia zaawansowane",
        "Wyjście",
    ];
    let fmt_options = ["lista", "przecinki", "markdown"];

    while running {
        println!("\n--- GENERATOR MORFOLOGICZNY ---");

        let choice = Select::new("Wybierz tryb:", menu_options.to_vec())
            .with_page_size(10)
            .prompt();

        match choice {
            Ok(opt) => match opt {
                "Generuj" => {
                    let names = generate_morphology(&pattern);
                    format_result(&names, &output_format);
                }
                "Zmień wzór morfologiczny" => {
                    let input = Text::new("Podaj wzór morfologiczny:")
                        .with_initial_value(&pattern)
                        .prompt()
                        .unwrap();
                    pattern = input.trim().to_string();
                }
                "Zmień formę rezultatu" => {
                    //let fmt_options = vec!["lista", "przecinki", "markdown"];
                    let fmt_choice = Select::new("Wybierz formę rezultatu:", fmt_options.to_vec())
                        .prompt()
                        .unwrap();

                    // Mapujemy wybór tekstowy na nasz bezpieczny Enum
                    output_format = match fmt_choice {
                        "przecinki" => OutputFormat::Przecinki,
                        "markdown" => OutputFormat::Markdown,
                        _ => OutputFormat::Lista,
                    };
                }
                "Zmień sortowanie/grupowanie" => {
                    println!("Opcje sortowania/grupowania na razie puste.");
                }
                "Ustawienia zaawansowane" => {
                    println!("Opcje zaawansowane na razie puste.");
                }
                "Wyjście" => {
                    running = false;
                }
                _ => {}
            },
            Err(_) => {
                println!("Błąd wyboru, wychodzimy.");
                running = false;
            }
        }
    }
}

```

## Plik-RustLibMod_02_00: `src/lib/cli/mod.rs`

```rust
pub mod menu;

```

## Plik-RustLibPub_03_01: `src/lib/geneteka/data_raw_modele.rs`

```rust
use serde::Deserialize;

// Główna struktura trzymająca całą tablicę [[rekord]]
#[derive(Debug, Deserialize)]
pub struct BazaGeneteki {
    pub rekord: Vec<Rekord>,
}

#[derive(Debug, Deserialize)]
pub struct Rekord {
    pub lp: u32,
    pub miejsce: Miejsce,
    pub roczniki: Roczniki,
}

#[derive(Debug, Deserialize)]
pub struct Miejsce {
    pub parafia: Vec<String>, // np. ["Baturyn", "6065"]
    pub obszar: Vec<String>,  // np. ["Białoruś", "22br"]
    pub lonlat: Vec<f64>,     // np. [27.860656, 54.054596]
}

#[derive(Debug, Deserialize)]
pub struct Roczniki {
    #[serde(default)]
    pub u: Vec<u32>,
    #[serde(default)]
    pub m: Vec<u32>,
    #[serde(default)]
    pub z: Vec<u32>,
}

```

## Plik-RustLibMod_03_00: `src/lib/geneteka/mod.rs`

```rust
// 1. Wspólne fundamenty
pub mod data_raw_modele;
pub mod modele;
pub mod parser_rocznikow;
pub mod pobrany_najnowszy;

// 2. Moduł: Mapa Google (KML)
pub mod parser_kml;
pub mod pobieracz_kml;

// 3. Moduł: Rejestry Geneteki (HTML)
pub mod parser_html;
pub mod pobieracz_html;

```

## Plik-RustLibPub_03_02: `src/lib/geneteka/modele.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rekord {
    pub lp: usize,
    pub miejsce: Miejsce,
    pub roczniki: Roczniki,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Miejsce {
    pub lonlat: [f64; 2],
    pub obszar: Vec<String>,
    pub parafia: Vec<String>,
}

// Nowa, lepsza nazwa!
#[derive(Debug, Serialize, Deserialize)]
pub struct Roczniki {
    pub u: Vec<u16>,
    pub m: Vec<u16>,
    pub z: Vec<u16>,
}

impl Roczniki {
    pub fn nowy() -> Self {
        Self {
            u: vec![],
            m: vec![],
            z: vec![],
        }
    }
}

```

## Plik-RustLibPub_03_03: `src/lib/geneteka/parser_html.rs`

```rust
use anyhow::{Context, Result};
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::fs;

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

            if teren == "Tereny" || teren.is_empty() {
                continue;
            }

            // Przeszukujemy 4. kolumnę w poszukiwaniu parafii (tagi <a>)
            for a in tds[3].select(&a_selector) {
                let href = a.value().attr("href").unwrap_or("");
                let title = a.value().attr("title").unwrap_or(""); // Tu kryją się roczniki!
                let klasa = a.value().attr("class").unwrap_or(""); // B, S, D
                let nazwa_parafii = a.text().collect::<String>().trim().to_string();

                let kod_rid = wyciagnij_parametr(href, "rid").unwrap_or_default();
                let kod_w = wyciagnij_parametr(href, "w").unwrap_or_default();

                if kod_w.is_empty() || nazwa_parafii.is_empty() {
                    continue;
                }

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
        a.miejsce
            .obszar
            .get(1)
            .unwrap_or(&String::new())
            .cmp(b.miejsce.obszar.get(1).unwrap_or(&String::new()))
            .then_with(|| a.miejsce.parafia[0].cmp(&b.miejsce.parafia[0]))
    });

    for (i, rekord) in wyniki.iter_mut().enumerate() {
        rekord.lp = i;
        rekord.roczniki.u.sort_unstable();
        rekord.roczniki.u.dedup();
        rekord.roczniki.m.sort_unstable();
        rekord.roczniki.m.dedup();
        rekord.roczniki.z.sort_unstable();
        rekord.roczniki.z.dedup();
    }

    println!(
        "[+] Przetworzono {} unikalnych parafii z tabeli HTML.",
        wyniki.len()
    );

    // --- Zapis do JSON ---
    let json_linie: Vec<String> = wyniki
        .iter()
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

```

## Plik-RustLibPub_03_04: `src/lib/geneteka/parser_kml.rs`

```rust
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

```

## Plik-RustLibPub_03_05: `src/lib/geneteka/parser_rocznikow.rs`

```rust
/// Funkcja, która tłumaczy "1846,61-62" na czystą listę lat: [1846, 1861, 1862]
pub fn rozkoduj_lata(dane: &str) -> Vec<u16> {
    let mut wyniki = Vec::new();
    let mut ostatnie_stulecie = 1800; // Baza, na wypadek gdyby pierwsza liczba była 2-cyfrowa

    for czesc in dane.split(',') {
        let czesc = czesc.trim();
        if czesc.is_empty() {
            continue;
        }

        if czesc.contains('-') {
            // Mamy zakres np. 1846-1850 lub 61-62
            let granice: Vec<&str> = czesc.split('-').collect();
            if granice.len() == 2 {
                let start = parsuj_rok(granice[0], &mut ostatnie_stulecie);
                let koniec = parsuj_rok(granice[1], &mut ostatnie_stulecie);
                if start > 0 && koniec >= start {
                    wyniki.extend(start..=koniec);
                }
            }
        } else {
            // Pojedynczy rok np. 1846 lub 61
            let rok = parsuj_rok(czesc, &mut ostatnie_stulecie);
            if rok > 0 {
                wyniki.push(rok);
            }
        }
    }

    // Sortowanie i usuwanie duplikatów (uniqueBy + sort z Twojego JS-a)
    wyniki.sort_unstable();
    wyniki.dedup();
    wyniki
}

/// Rozpoznaje czy rok jest 4-cyfrowy (1846) czy 2-cyfrowy (61)
fn parsuj_rok(tekst: &str, ostatnie_stulecie: &mut u16) -> u16 {
    let tekst = tekst.trim();
    if tekst.len() == 4 {
        if let Ok(rok) = tekst.parse::<u16>() {
            *ostatnie_stulecie = (rok / 100) * 100; // Zapamiętuje np. 1800 z 1846
            return rok;
        }
    } else if tekst.len() == 2
        && let Ok(skrot) = tekst.parse::<u16>()
    {
        return *ostatnie_stulecie + skrot; // np. 1800 + 61 = 1861
    }
    0 // Błąd parsowania
}

```

## Plik-RustLibPub_03_06: `src/lib/geneteka/pobieracz_html.rs`

```rust
use anyhow::{Context, Result};
use chrono::Local;
use reqwest::header;
use std::fs;
use std::path::Path;

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
    headers.insert(
        header::ACCEPT_LANGUAGE,
        "pl-PL,pl;q=0.9,en-US;q=0.8,en;q=0.7".parse().unwrap(),
    );
    // Udajemy, że weszliśmy z głównej strony Geneteki
    headers.insert(
        header::REFERER,
        "https://geneteka.genealodzy.pl/".parse().unwrap(),
    );
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
    let odpowiedz = klient
        .get(url)
        .send()
        .await
        .context("Błąd wysyłania zapytania do Geneteki")?;

    if odpowiedz.status().is_success() {
        // Pobieramy CAŁY tekst odpowiedzi
        let tekst_html = odpowiedz.text().await.context("Błąd odczytu tekstu HTML")?;

        // 5. ZABEZPIECZENIE: Sprawdzamy, czy serwer nie rzucił błędem bazy zamiast wysłać tabelę!
        if tekst_html.contains("Connect to database error") {
            anyhow::bail!(
                "Serwer Geneteki jest przeciążony i wyrzucił błąd bazy danych (Connect to database error). Spróbuj ponownie później lub użyj pliku pobranego ręcznie."
            );
        }

        // 6. Zapisujemy ten potężny plik na dysk
        fs::write(&pelna_sciezka, &tekst_html).context("Błąd zapisu pliku HTML")?;

        Ok(pelna_sciezka)
    } else {
        anyhow::bail!(
            "Serwer odrzucił połączenie. Kod błędu HTTP: {}",
            odpowiedz.status()
        )
    }
}

```

## Plik-RustLibPub_03_07: `src/lib/geneteka/pobieracz_kml.rs`

```rust
use anyhow::{Context, Result};
use chrono::Local;
use std::fs;
use std::io::Write;
use std::path::Path;

pub async fn pobierz_zakres() -> Result<String> {
    let url = "https://www.google.com/maps/d/kml?mid=1Ig20G_J_1vRHY4aYPmyLj2VqfiDsLkNJ&forcekml=1";

    let dir_path = "./data/genealodzy-geneteka/raw";
    let dzisiaj = Local::now().format("%Y-%m-%d").to_string();
    let nazwa_pliku = format!("mapa_{}.kml", dzisiaj);
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

```

## Plik-RustLibPub_03_08: `src/lib/geneteka/pobrany_najnowszy.rs`

```rust
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

```

## Plik-RustLibPub_04_01: `src/lib/logic/format.rs`

```rust
use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)] // ValueEnum pozwala wpiąć to od razu w CLI (clap)
pub enum OutputFormat {
    Lista,
    Przecinki,
    Markdown,
}

/// Wyświetlanie wyników w wybranym formacie
pub fn format_result(names: &[String], format: &OutputFormat) {
    match format {
        OutputFormat::Lista => {
            for n in names {
                println!("- {}", n);
            }
        }
        OutputFormat::Przecinki => println!("{}", names.join(", ")),
        OutputFormat::Markdown => {
            println!("| Wyraz |\n| :--- |");
            for n in names {
                println!("| {} |", n);
            }
        }
    }
}

```

## Plik-RustLibMod_04_00: `src/lib/logic/mod.rs`

```rust
pub mod format;
pub mod morphology;
pub mod utils;

```

## Plik-RustLibPub_04_02: `src/lib/logic/morphology.rs`

```rust
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

```

## Plik-RustLibPub_04_03: `src/lib/logic/utils.rs`

```rust
use anyhow::Result;
use std::process;

/// Funkcja pomocnicza, która wypakowuje ścieżkę z wyniku lub elegancko kończy program w razie błędu.
pub fn obsluz_wynik_pobierania(wynik: Result<String>) -> String {
    match wynik {
        Ok(sciezka) => sciezka,
        Err(blad) => {
            eprintln!("[-] Niestety, operacja zawiodła: {:?}", blad);
            // Zamykamy program z kodem błędu 1 (standard w systemach operacyjnych dla błędu)
            process::exit(1);
        }
    }
}

/// Funkcja pomocnicza do obsługi wyników parsowania (nie zwraca danych, tylko komunikaty)
pub fn obsluz_wynik_parsowania(wynik: Result<()>, msg_sukces: &str, msg_blad: &str) {
    match wynik {
        Ok(_) => println!("[*] {}", msg_sukces),
        Err(e) => {
            eprintln!("[-] {}: {:?}", msg_blad, e);
            process::exit(1);
        }
    }
}

```

## Plik-RustLibMod_00: `src/lib/mod.rs`

```rust
pub mod atlas;
pub mod cli;
pub mod logic;
pub mod pliki;
pub mod window;

pub mod geneteka;

```

## Plik-RustLibMod_05_00: `src/lib/pliki/mod.rs`

```rust
pub mod path_data_naturalearth;

```

## Plik-RustLibPub_05_01: `src/lib/pliki/path_data_naturalearth.rs`

```rust
pub const DIR_VECT_PHY: &str = "data-naturalearth/Vector-Physical";

pub const PATH_RAMKA: &str = "data-naturalearth/Vector-Physical/ne__ramka.geojson";

// --- LINIE BRZEGOWE (COASTLINE) ---
pub const PATH_COASTLINE_110M: &str = "data-naturalearth/Vector-Physical/ne_110m_coastline.geojson";
pub const PATH_COASTLINE_50M: &str = "data-naturalearth/Vector-Physical/ne_50m_coastline.geojson";
pub const PATH_COASTLINE_10M: &str = "data-naturalearth/Vector-Physical/ne_10m_coastline.geojson";

// --- LODOWCE (GLACIATED AREAS) ---
pub const PATH_GLACIERS_110M: &str =
    "data-naturalearth/Vector-Physical/ne_110m_glaciated_areas.geojson";
pub const PATH_GLACIERS_50M: &str =
    "data-naturalearth/Vector-Physical/ne_50m_glaciated_areas.geojson";
pub const PATH_GLACIERS_10M: &str =
    "data-naturalearth/Vector-Physical/ne_10m_glaciated_areas.geojson";

// --- JEZIORA (LAKES) ---
pub const PATH_LAKES_110M: &str = "data-naturalearth/Vector-Physical/ne_110m_lakes.geojson";
pub const PATH_LAKES_50M: &str = "data-naturalearth/Vector-Physical/ne_50m_lakes.geojson";
pub const PATH_LAKES_10M: &str = "data-naturalearth/Vector-Physical/ne_10m_lakes.geojson";

// Jeziora szczegółowe (tylko 10m i 50m)
pub const PATH_LAKES_EUROPE_10M: &str =
    "data-naturalearth/Vector-Physical/ne_10m_lakes_europe.geojson";
pub const PATH_LAKES_NORTH_AMERICA_10M: &str =
    "data-naturalearth/Vector-Physical/ne_10m_lakes_north_america.geojson";
pub const PATH_LAKES_AUSTRALIA_10M: &str =
    "data-naturalearth/Vector-Physical/ne_10m_lakes_australia.geojson";
pub const PATH_LAKES_HISTORIC_50M: &str =
    "data-naturalearth/Vector-Physical/ne_50m_lakes_historic.geojson";
pub const PATH_LAKES_HISTORIC_10M: &str =
    "data-naturalearth/Vector-Physical/ne_10m_lakes_historic.geojson";
pub const PATH_LAKES_PLUVIAL_10M: &str =
    "data-naturalearth/Vector-Physical/ne_10m_lakes_pluvial.geojson";

// --- RZEKI (RIVERS) ---
pub const PATH_RIVERS_LAKE_CENTER_110M: &str =
    "data-naturalearth/Vector-Physical/ne_110m_rivers_lake_centerlines.geojson";
pub const PATH_RIVERS_LAKE_CENTER_50M: &str =
    "data-naturalearth/Vector-Physical/ne_50m_rivers_lake_centerlines.geojson";
pub const PATH_RIVERS_LAKE_CENTER_10M: &str =
    "data-naturalearth/Vector-Physical/ne_10m_rivers_lake_centerlines.geojson";

// Rzeki ze skalowaniem (scale rank)
pub const PATH_RIVERS_SCALE_50M: &str =
    "data-naturalearth/Vector-Physical/ne_50m_rivers_lake_centerlines_scale_rank.geojson";
pub const PATH_RIVERS_SCALE_10M: &str =
    "data-naturalearth/Vector-Physical/ne_10m_rivers_lake_centerlines_scale_rank.geojson";

// Rzeki regionalne (tylko 10m)
pub const PATH_RIVERS_EUROPE_10M: &str =
    "data-naturalearth/Vector-Physical/ne_10m_rivers_europe.geojson";
pub const PATH_RIVERS_NORTH_AMERICA_10M: &str =
    "data-naturalearth/Vector-Physical/ne_10m_rivers_north_america.geojson";
pub const PATH_RIVERS_AUSTRALIA_10M: &str =
    "data-naturalearth/Vector-Physical/ne_10m_rivers_australia.geojson";

// --- WYSPY (MINOR ISLANDS) ---
pub const PATH_MINOR_ISLANDS_10M: &str =
    "data-naturalearth/Vector-Physical/ne_10m_minor_islands.geojson";
pub const PATH_MINOR_ISLANDS_COAST_10M: &str =
    "data-naturalearth/Vector-Physical/ne_10m_minor_islands_coastline.geojson";

```

## Plik-RustLibPub_06: `src/lib/window.rs`

```rust
// ./src/lib/window.rs

use i_slint_backend_winit::WinitWindowAccessor; // pozwala nam dobrać się do natywnego okna
use slint::{SharedString, Window};
use winit::window::ResizeDirection;

pub fn minimize(window: &Window) {
    window.set_minimized(true);
}

pub fn start_drag(window: &Window) {
    // Używamy "with_winit_window", aby uzyskać dostęp do funkcji systemowych
    window.with_winit_window(|winit_window| {
        // To wywołuje natywne przesuwanie okna systemu Windows/Linux/macOS
        // if let Err(e) = winit_window.drag_window() {
        //     eprintln!("Błąd podczas przesuwania okna: {}", e);
        // }
        // To odpalamy TYLKO dla myszki
        let _ = winit_window.drag_window();
    });
}

// Ręczne przesuwanie o zadaną wartość (dla dotyku)
pub fn window_move(window: &Window, delta_x: f32, delta_y: f32) {
    window.with_winit_window(|winit_window| {
        if let Ok(current_pos) = winit_window.outer_position() {
            let scale_factor = winit_window.scale_factor();
            // Przeliczamy logiczne piksele Slinta na fizyczne piksele ekranu
            let dx = (delta_x as f64 * scale_factor) as i32;
            let dy = (delta_y as f64 * scale_factor) as i32;

            let new_x = current_pos.x + dx;
            let new_y = current_pos.y + dy;

            winit_window.set_outer_position(winit::dpi::PhysicalPosition::new(new_x, new_y));
        }
    });
}

pub fn window_resize(window: &Window, direction: SharedString) {
    window.with_winit_window(|winit_window| {
        let dir = match direction.as_str() {
            "n" => ResizeDirection::North,
            "s" => ResizeDirection::South,
            "e" => ResizeDirection::East,
            "w" => ResizeDirection::West,
            "ne" => ResizeDirection::NorthEast,
            "nw" => ResizeDirection::NorthWest,
            "se" => ResizeDirection::SouthEast,
            "sw" => ResizeDirection::SouthWest,
            _ => return, // Nieznany kierunek - ignorujemy
        };
        let _ = winit_window.drag_resize_window(dir);

        if let Err(e) = winit_window.drag_resize_window(dir) {
            eprintln!("Resize error: {}", e);
        }
    });
}

#[macro_export]
macro_rules! setup_window_ctrl_bindings {
    ($ui:expr, $ui_type:ty) => {
        // Wyciągamy globalny kontroler z podanego UI
        let logika = $ui.global::<OknoLogika>();

        logika.on_zamykanie(|| {
            let _ = slint::quit_event_loop();
        });

        // Wymuszamy na kompilatorze konkretny typ Weak<$ui_type>
        let ui_weak_min: slint::Weak<$ui_type> = slint::ComponentHandle::as_weak(&$ui);
        logika.on_ukrywanie(move || {
            if let Some(ui) = ui_weak_min.upgrade() {
                slint::ComponentHandle::window(&ui).set_minimized(true);
            }
        });

        let ui_weak_move: slint::Weak<$ui_type> = slint::ComponentHandle::as_weak(&$ui);
        logika.on_przesuwanie(move |dx, dy| {
            if let Some(ui) = ui_weak_move.upgrade() {
                if dx == 0.0 && dy == 0.0 {
                    $crate::window::start_drag(slint::ComponentHandle::window(&ui));
                } else {
                    $crate::window::window_move(slint::ComponentHandle::window(&ui), dx, dy);
                }
            }
        });

        let ui_weak_resize: slint::Weak<$ui_type> = slint::ComponentHandle::as_weak(&$ui);
        logika.on_wymiarowanie(move |direction| {
            if let Some(ui) = ui_weak_resize.upgrade() {
                $crate::window::window_resize(slint::ComponentHandle::window(&ui), direction);
            }
        });
    };
}

```

