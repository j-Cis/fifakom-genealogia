# RAPORT KODU: doc/code-rust-app.md

## Plik-TomlCargo: `Cargo.toml`

```toml
[package]
name = "fifakom-genealogia"
version = "0.1.5"
readme = "README.md"
repository = "https://github.com/j-Cis/fifakom-genealogia"
description = "Zestaw narzędzi i sztuczek pomocnych zaawansowanym genealogom."
authors = ["Jan Roman Cisowski „Gepiden”"]
edition = "2024"       # Używana edycja języka Rust (nowoczesne funkcje i resolver)
rust-version = "1.93.1" # Minimalna wersja kompilatora wymagana do zbudowania projektu
resolver = "3"         # Sposób, w jaki Cargo rozwiązuje zależności (3 to standard dla edycji 2024)

# ZALEŻNOŚCI GŁÓWNE (Kompilowane do końcowego programu)
[dependencies]
# --- Narzędzia ogólne i obsługa błędów ---
anyhow = "1.0.102"      # Prosta i elastyczna obsługa błędów (typ anyhow::Result)
itertools = "0.14.0"    # Dodatkowe, potężne metody dla iteratorów (często używane przy kombinacjach/kartezjańskich)
regex = "1.12.3"        # Obsługa wyrażeń regularnych do analizy i formatowania tekstu
chrono = "0.4.44"       # Obsługa dat i czasu (używane np. do dodawania daty do nazw pobieranych plików)

# --- Interfejs Linii Poleceń (CLI) ---
clap = { version = "4.5.60", features = ["derive"] } # Potężny parser argumentów z wiersza poleceń
inquire = "0.9.4"       # Interaktywne prompty/menu w terminalu (pytania, listy wyboru)
colored = "3.1.1"       # Kolorowanie tekstu w konsoli (żeby komunikaty wyróżniały się wizualnie)
atty = "0.2.14"         # Sprawdza, czy program działa w prawdziwym terminalu (uwaga: w nowym Rust można użyć std::io::IsTerminal)

# --- Graficzny Interfejs Użytkownika (GUI) ---
slint = "1.15.1"               # Główna biblioteka frameworka GUI Slint
i-slint-backend-winit = "1.15.1" # Wewnętrzny backend Slinta oparty na Winit (obsługa okien i wejścia)
winit = "0.30.13"              # Niskopoziomowa biblioteka do tworzenia okien (używana pod spodem przez Slinta)
arboard = "3.6.1"              # Biblioteka do obsługi schowka (np. kopiowanie wyników działania programu)

# --- Sieć i Asynchroniczność (Pobieranie danych z Geneteki) ---
tokio = { version = "1.50.0", features = ["rt-multi-thread", "macros"] } # Środowisko asynchroniczne niezbędne do działania reqwest
reqwest = "0.13.2"      # Klient HTTP do pobierania plików KML (Google Maps) i stron HTML (Geneteka)
url = "2.5.8"           # Biblioteka do łatwej analizy i wyciągania parametrów z linków (np. rid i w)

# --- Przetwarzanie i Serializacja Danych (XML, HTML, JSON) ---
serde = { version = "1.0.228", features = ["derive"] } # Potężny framework do definiowania struktur danych (Serialize/Deserialize)
serde_json = "1.0.149"  # Narzędzie do konwersji naszych struktur w Rusta na pliki JSON i odwrotnie
roxmltree = "0.21.1"    # Ultraszybki parser drzewa XML – używany do czytania tagów z map KML
scraper = "0.25.0"      # Parser HTML z obsługą selektorów CSS (wyciąganie danych prosto z tabel na stronie WWW)

# ---- INNE NARZĘDZIA ----
toml = "1.0.6"
tiny-skia = "0.12.0"
geojson = "0.24.2"

# ZALEŻNOŚCI BUDOWANIA (Używane TYLKO przez plik build.rs przed główną kompilacją)
[build-dependencies]
slint-build = "1.15.1"  # Kompilator Slinta: tłumaczy pliki .slint na kod Rusta podczas etapu budowania

# KONFIGURACJA BIBLIOTEKI
[lib]
name = "fifak_lib"
path = "src/lib/mod.rs" # Wskazuje, gdzie znajduje się główne "jądro" Twojej aplikacji (biblioteka wewnętrzna)

# KONFIGURACJA PLIKÓW WYKONYWALNYCH (Pojedyncze programy bazujące na fifak_lib)

[[bin]]
name = "PobieraczGenetekaZakresMapaGET"  
path = "src/bin/job/pobieracz_geneteka_zakres_mapa__get.rs"

[[bin]]
name = "PobieraczGenetekaZakresHtmlGET"  
path = "src/bin/job/pobieracz_geneteka_zakres_html__get.rs"

[[bin]]
name = "DokumentacjaEXTRACT"  
path = "src/bin/job/dokumentacja_extract.rs"

[[bin]]
name = "AppKartezjanProduktCLI"                    
path = "src/bin/cli/app_kartezjan_produkt__cli.rs" 

[[bin]]
name = "AppKartezjanProduktTUI"                    
path = "src/bin/tui/app_kartezjan_produkt__tui.rs" 

[[bin]]
name = "AppKartezjanProduktGUI"                
path = "src/bin/gui/app_kartezjan_produkt__gui.rs"

[[bin]]
name = "AppGenetekaZakresGUI"  
path = "src/bin/gui/app_geneteka_zakres__gui.rs"

```

## Plik-TomlMakefile: `Makefile.toml`

```toml
[config]
skip_core_tasks = true

# ==========================================
# KOMENDY INFORMACYJNE
# ==========================================

[tasks.h]
description = "Wyświetla listę dostępnych komend"
command = "powershell"
args = [
    "-NoProfile",
    "-Command",
    '''
Write-Host "=================================================" -ForegroundColor Cyan
Write-Host " DOSTĘPNE KOMENDY (cargo make):" -ForegroundColor Yellow
Write-Host "-------------------------------------------------" -ForegroundColor Cyan
Write-Host "  cargo make h         : Wyświetla tę listę komend"
Write-Host "  cargo make bins      : Wyświetla listę aplikacji z Cargo.toml"
Write-Host "  cargo make bins-b    : Buduje WSZYSTKIE apki (Debug + Release) BEZ kopiowania"
Write-Host "  cargo make bins-get  : Tylko KOPIUJE to, co już zbudowane (Force)"
Write-Host "  cargo make b-r-all     : Buduje wszystkie apki (Release) i kopiuje"
Write-Host "  cargo make b-d-all     : Buduje wszystkie apki (Debug) i kopiuje"
Write-Host "  cargo make b-r [nazwa] : Buduje JEDNĄ apkę (Release) i kopiuje"
Write-Host "  cargo make b-d [nazwa] : Buduje JEDNĄ apkę (Debug) i kopiuje"
Write-Host "=================================================" -ForegroundColor Cyan
''',
]

[tasks.bins]
description = "Wypisuje listę aplikacji pobraną bezpośrednio z Cargo.toml"
command = "powershell"
args = [
    "-NoProfile",
    "-Command",
    '''
$content = [System.IO.File]::ReadAllText("Cargo.toml")
$matches = [regex]::Matches($content, '\[\[bin\]\]\s*name\s*=\s*"([^"]+)"')
Write-Host "Lista aplikacji znalezionych w Cargo.toml:" -ForegroundColor Yellow
foreach ($m in $matches) { Write-Host " - $($m.Groups[1].Value)" -ForegroundColor Cyan }
''',
]


# ==========================================
# TASKS KOPIOWANIA (Dynamiczne z Cargo.toml)
# ==========================================

[tasks.dist-r]
description = "Kopiuje pliki .exe z target/release do dist"
command = "powershell"
args = [
    "-NoProfile",
    "-Command",
    '''
$targetDir = "dist/win/release"
if (!(Test-Path $targetDir)) { New-Item -ItemType Directory -Force -Path $targetDir | Out-Null }

$content = [System.IO.File]::ReadAllText("Cargo.toml")
$matches = [regex]::Matches($content, '\[\[bin\]\]\s*name\s*=\s*"([^"]+)"')

foreach ($m in $matches) {
    $bin = $m.Groups[1].Value
    $src = "target/release/${bin}.exe"
    if (Test-Path $src) {
        Copy-Item $src "$targetDir/" -Force
        Write-Host "Skopiowano (Release): $bin.exe" -ForegroundColor Green
    }
}
''',
]

[tasks.dist-d]
description = "Kopiuje pliki .exe z target/debug do dist"
command = "powershell"
args = [
    "-NoProfile",
    "-Command",
    '''
$targetDir = "dist/win/debug"
if (!(Test-Path $targetDir)) { New-Item -ItemType Directory -Force -Path $targetDir | Out-Null }

$content = [System.IO.File]::ReadAllText("Cargo.toml")
$matches = [regex]::Matches($content, '\[\[bin\]\]\s*name\s*=\s*"([^"]+)"')

foreach ($m in $matches) {
    $bin = $m.Groups[1].Value
    $src = "target/debug/${bin}.exe"
    if (Test-Path $src) {
        Copy-Item $src "$targetDir/" -Force
        Write-Host "Skopiowano (Debug)  : $bin.exe" -ForegroundColor Green
    }
}
''',
]

[tasks.bins-get]
description = "Kopiuje wszystko do dist (Debug + Release)"
dependencies = ["dist-d", "dist-r"]


# ==========================================
# ZADANIA BUDOWANIA (Baza dla Cargo)
# ==========================================

[tasks.cargo-b-r-all]
command = "cargo"
args = ["build", "--bins", "--release"]

[tasks.cargo-b-d-all]
command = "cargo"
args = ["build", "--bins"]

[tasks.cargo-b-r-single]
command = "cargo"
args = ["build", "--release", "--bin", "${@}"]

[tasks.cargo-b-d-single]
command = "cargo"
args = ["build", "--bin", "${@}"]


# ==========================================
# GŁÓWNE KOMENDY UŻYTKOWNIKA
# ==========================================

# [tasks.bins-b]
# description = "Buduje WSZYSTKIE binarki w Debug i Release, po czym kopiuje"
# dependencies = ["b-d-all", "b-r-all"]

[tasks.bins-b]
description = "Buduje WSZYSTKIE binarki w Debug i Release (BEZ KOPIOWANIA)"
dependencies = ["cargo-b-d-all", "cargo-b-r-all"]

[tasks.b-r-all]
description = "Buduje wszystkie binarki (release) i kopiuje"
dependencies = ["cargo-b-r-all", "dist-r"]

[tasks.b-d-all]
description = "Buduje wszystkie binarki (debug) i kopiuje"
dependencies = ["cargo-b-d-all", "dist-d"]

[tasks.b-r]
description = "Buduje jedną binarkę (release) i kopiuje"
dependencies = ["cargo-b-r-single", "dist-r"]

[tasks.b-d]
description = "Buduje jedną binarkę (debug) i kopiuje"
dependencies = ["cargo-b-d-single", "dist-d"]

```

## Plik-RustBuild: `build.rs`

```rust
fn main() {
    //let config = slint_build::CompilerConfiguration::default();
    slint_build::compile("src/ui/index.slint").expect("Błąd kompilacji interfejsu (index.slint)");
}

```

## Plik-RustBinCli_01: `src/bin/cli/app_kartezjan_produkt__cli.rs`

```rust
use clap::Parser;

use fifak_lib::logic::format::OutputFormat;
use fifak_lib::logic::format::format_result;
use fifak_lib::logic::morphology::generate_morphology;

#[derive(Parser)]
#[command(name = "Generator Morfologiczny - CLI")]
#[command(author = "Jaśko")]
#[command(version = "0.1.1")]
#[command(about = "Szybki generator wyrazów z wiersza poleceń")]
struct Cli {
    /// Wzór morfologiczny, np "(Zg|Sg|Zk|Sk)o(d|ds|dz)"
    #[arg(required = true)]
    pattern: String,

    /// Format wyjściowy
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Lista)]
    format: OutputFormat,
}

fn main() {
    // Od razu parsujemy i generujemy, bez zbędnych matchy
    let cli = Cli::parse();

    let names = generate_morphology(&cli.pattern);
    format_result(&names, &cli.format);
}

```

## Plik-RustBinGui_01: `src/bin/gui/app_geneteka_zakres__gui.rs`

```rust
#![windows_subsystem = "windows"]

use anyhow::Result;

use slint::ComponentHandle;
use std::rc::Rc; // UWAGA: Usunąłem stąd VecModel, bo już go nie używamy!

use fifak_lib::atlas::{MapProjection, generate_map_data};
use fifak_lib::setup_window_ctrl_bindings;

slint::include_modules!();

fn main() -> Result<()> {
    let sciezka_danych = "./data/genealodzy-geneteka/raw";

    let baza =
        fifak_lib::geneteka::pobrany_najnowszy::laduj_baze(sciezka_danych).unwrap_or_else(|| {
            println!(
                "Nie znaleziono danych w {}. Tworzę pustą mapę.",
                sciezka_danych
            );
            fifak_lib::geneteka::data_raw_modele::BazaGeneteki { rekord: vec![] }
        });

    println!("Wczytano {} parafii!", baza.rekord.len());

    let ui = AppGenetekaZakres::new()?;
    setup_window_ctrl_bindings!(ui, AppGenetekaZakres);

    // ==========================================
    // LOGIKA MAPY -> przeniesiona do biblioteki atlas!
    // Generujemy dane i od razu pakujemy w Rc, by były dostępne dla silnika rysującego
    //
    // ODKOMENTUJ TO, jeśli chcesz wrócić do dynamicznych marginesów:
    // let map_data = Rc::new(generate_map_data(&baza.rekord, MapProjection::Dynamic { margin: 0.05 }));
    //
    // ZOSTAW TO, jeśli używamy mapy całego świata:
    let map_data = Rc::new(generate_map_data(&baza.rekord, MapProjection::ProjWgs84));
    // ==========================================

    // ------------------ TRANSFER GRANIC DO SLINTA ------------------
    ui.set_geo_min_lon(map_data.min_lon as f32);
    ui.set_geo_max_lon(map_data.max_lon as f32);
    ui.set_geo_min_lat(map_data.min_lat as f32);
    ui.set_geo_max_lat(map_data.max_lat as f32);
    // ---------------------------------------------------------------

    // ==========================================
    // NOWOŚĆ: SILNIK RENDERUJĄCY W RUŚCIE (TINY-SKIA)
    // To zastępuje całkowicie pętlę "for pt in map_data.points..."
    // ==========================================
    let ui_handle = ui.as_weak();
    let map_data_render = Rc::clone(&map_data); // Klonujemy referencję dla closure

    ui.on_camera_changed(move |w, h, offset_x, offset_y, zoom, rot| {
        if let Some(ui_ref) = ui_handle.upgrade() {
            // Rust błyskawicznie rysuje płótno na podstawie widoku z kamery:
            let image_frame = fifak_lib::atlas::renderer::render_frame(
                w as u32,
                h as u32,
                &map_data_render,
                offset_x,
                offset_y,
                zoom,
                rot,
            );
            // I wysyła jedną gotową, lekką klatkę do Slinta:
            ui_ref.set_map_frame(image_frame);
        }
    });
    // ==========================================

    ui.on_search(|text| {
        println!("[*] Szukamy: {}", text);
    });

    ui.run()?;
    Ok(())
}

```

## Plik-RustBinGui_02: `src/bin/gui/app_kartezjan_produkt__gui.rs`

```rust
#![windows_subsystem = "windows"]

use anyhow::Result;
use arboard::Clipboard;
use fifak_lib::logic::morphology::generate_morphology;
use fifak_lib::setup_window_ctrl_bindings;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

slint::include_modules!();

fn main() -> Result<()> {
    let ui = AppKartezjanProdukt::new()?;
    setup_window_ctrl_bindings!(ui, AppKartezjanProdukt);

    // 1. Generowanie
    let ui_weak_send = ui.as_weak();
    ui.on_send(move |pattern| {
        if let Some(ui) = ui_weak_send.upgrade() {
            let results = generate_morphology(pattern.as_str());
            let model: Vec<SharedString> = results.iter().map(SharedString::from).collect();
            ui.set_results(ModelRc::new(VecModel::from(model)));

            let joined = results.join(", ");
            ui.set_joined_text(SharedString::from(joined));
        }
    });

    // 2. Schowek
    let ui_weak_copy = ui.as_weak();
    ui.on_copy_to_clipboard(move |mode| {
        if let Some(ui) = ui_weak_copy.upgrade() {
            let text_to_copy = if mode == "list" {
                let model = ui.get_results();
                let mut items: Vec<String> = Vec::new();
                for i in 0..model.row_count() {
                    if let Some(val) = model.row_data(i) {
                        items.push(val.to_string());
                    }
                }
                items.join("\n")
            } else {
                ui.get_joined_text().to_string()
            };

            if let Ok(mut cb) = Clipboard::new() {
                let _ = cb.set_text(text_to_copy);
            }
        }
    });

    ui.run()?;
    Ok(())
}

```

## Plik-RustBinJob_01: `src/bin/job/dokumentacja_extract.rs`

```rust
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("[*] Uruchamiam zaawansowany generator dokumentacji...");

    if let Err(e) = fs::create_dir_all("doc") {
        eprintln!("[-] KRYTYCZNY BŁĄD: Nie udało się utworzyć folderu doc: {}", e);
        return;
    }

    // ==========================================
    // MAPOWANIE ID (Algorytm strukturalny)
    // ==========================================
    let mut id_map: HashMap<PathBuf, String> = HashMap::new();
    
    // 1. Pliki twardo zakodowane
    id_map.insert(PathBuf::from("Cargo.toml"), "TomlCargo".to_string());
    id_map.insert(PathBuf::from("Makefile.toml"), "TomlMakefile".to_string());
    id_map.insert(PathBuf::from("build.rs"), "RustBuild".to_string());
    id_map.insert(PathBuf::from("src/ui/index.slint"), "SlintIndex".to_string());

    // 2. Generowanie inteligentnych, hierarchicznych ID dla src/lib
    assign_lib_ids(Path::new("src/lib"), "", &mut id_map);

    // 3. Generowanie płaskich ID dla apek i komponentów (src/bin, src/ui)
    assign_flat_ids(Path::new("src/bin"), "RustBin", "rs", &mut id_map);
    assign_flat_ids(Path::new("src/ui"), "Slint", "slint", &mut id_map);

    // ==========================================
    // EKSTRAKCJA DO PLIKÓW
    // ==========================================
    
    // 1. DOKUMENTACJA SLINT (code-slint.md)
    let mut slint_files = vec![PathBuf::from("build.rs")];
    slint_files.extend(find_files(Path::new("src/ui"), "slint"));
    write_md("doc/code-slint.md", &slint_files, &id_map);

    // 2. DOKUMENTACJA RUST APP (code-rust-app.md)
    let mut app_files = vec![
        PathBuf::from("Cargo.toml"),
        PathBuf::from("Makefile.toml"),
        PathBuf::from("build.rs"),
    ];
    app_files.extend(find_files(Path::new("src/bin"), "rs"));
    write_md("doc/code-rust-app.md", &app_files, &id_map);

    // 3. DOKUMENTACJA RUST LIB (code-rust-lib.md)
    let lib_files = find_files(Path::new("src/lib"), "rs");
    write_md("doc/code-rust-lib.md", &lib_files, &id_map);

    println!("[+] Sukces! Pliki wygenerowano w folderze ./doc/");
}

// ---------------------------------------------------------
// FUNKCJE POMOCNICZE I ALGORYTMY ID
// ---------------------------------------------------------

fn assign_lib_ids(dir: &Path, current_prefix: &str, map: &mut HashMap<PathBuf, String>) {
    if !dir.is_dir() { return; }
    
    let Ok(read_dir) = fs::read_dir(dir) else { return; };
    let mut entries: Vec<_> = read_dir.flatten().collect();
    
    entries.sort_by_key(|e| e.file_name());

    let mut index = 1;

    if let Some(mod_idx) = entries.iter().position(|e| e.file_name() == "mod.rs") {
        let mod_entry = entries.remove(mod_idx);
        let mod_prefix = if current_prefix.is_empty() { String::new() } else { format!("{}_", current_prefix) };
        map.insert(mod_entry.path(), format!("RustLibMod_{}00", mod_prefix));
    }

    for entry in entries {
        let path = entry.path();
        let step_prefix = if current_prefix.is_empty() {
            format!("{:02}", index)
        } else {
            format!("{}_{:02}", current_prefix, index)
        };

        if path.is_dir() {
            assign_lib_ids(&path, &step_prefix, map);
            index += 1;
        } else if path.extension().unwrap_or_default() == "rs" {
            map.insert(path.clone(), format!("RustLibPub_{}", step_prefix));
            index += 1;
        }
    }
}

fn assign_flat_ids(dir: &Path, global_prefix: &str, ext: &str, map: &mut HashMap<PathBuf, String>) {
    let files = find_files(dir, ext);
    let mut counters: HashMap<String, usize> = HashMap::new();

    for path in files {
        if map.contains_key(&path) { continue; }

        let parent_name = path.parent()
            .and_then(|p| p.file_name())
            .unwrap_or_default()
            .to_string_lossy();

        let category = capitalize(&parent_name);
        let count = counters.entry(category.clone()).or_insert(1);

        let id = format!("{}{}_{:02}", global_prefix, category, count);
        map.insert(path, id);
        
        *count += 1;
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn find_files(dir: &Path, extension: &str) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if dir.is_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    result.extend(find_files(&path, extension));
                } else if path.extension().and_then(|s| s.to_str()) == Some(extension) {
                    result.push(path);
                }
            }
        }
    } else {
        eprintln!("[-] UWAGA: Ścieżka {} nie istnieje!", dir.display());
    }
    result.sort();
    result
}

fn write_md(out_path: &str, files: &[PathBuf], id_map: &HashMap<PathBuf, String>) {
    let mut content = String::new();
    content.push_str(&format!("# RAPORT KODU: {}\n\n", out_path));

    for path in files {
        let display_path = path.display().to_string().replace("\\", "/");
        
        if path.exists() {
            let file_content = fs::read_to_string(path).unwrap_or_else(|_| String::from("// BŁĄD ODCZYTU PLIKU"));
            
            // Pobieramy identyfikator z naszej mapy i wstawiamy BEZPOŚREDNIO w szablon
            let id = id_map.get(path).cloned().unwrap_or_else(|| "BrakID".to_string());
            
            let ext = path.extension().unwrap_or_default().to_string_lossy();
            let lang = match ext.as_ref() { "rs" => "rust", "slint" => "slint", "toml" => "toml", _ => "text" };
            
            // Formatowanie, które daje IDEALNY wynik: ## Plik-[ID]: [sciezka]
            content.push_str(&format!("## Plik-{}: `{}`\n\n```{}\n{}\n```\n\n", id, display_path, lang, file_content));
        } else {
            content.push_str(&format!("## BŁĄD: `{}` (Plik nie istnieje)\n\n", display_path));
        }
    }

    match fs::write(out_path, &content) {
        Ok(_) => println!(" [+] Wygenerowano: {}", out_path),
        Err(e) => eprintln!(" [-] Błąd zapisu {}: {}", out_path, e),
    }
}
```

## Plik-RustBinJob_02: `src/bin/job/pobieracz_geneteka_zakres_html__get.rs`

```rust
use fifak_lib::geneteka::parser_html::parsuj_html;
use fifak_lib::geneteka::pobieracz_html::pobierz_rejestry;
use fifak_lib::logic::utils::{obsluz_wynik_parsowania, obsluz_wynik_pobierania};

#[tokio::main]
async fn main() {
    println!("[*] Uruchamiam narzędzia Geneteki (Moduł Rejestrów HTML)...");

    // 1. Pobieranie lub wczytywanie pliku HTML
    let sciezka_html = obsluz_wynik_pobierania(pobierz_rejestry().await);

    println!("[+] Plik źródłowy HTML gotowy do analizy: {}", sciezka_html);

    // 2. Przygotowanie ścieżki docelowej
    // Wynik zapiszemy jako rejestry_YYYY-MM-DD.json
    let sciezka_json = sciezka_html.replace(".html", ".json");

    // 3. Odpalamy nasz "Kombajn do HTML-a"
    obsluz_wynik_parsowania(
        parsuj_html(&sciezka_html, &sciezka_json),
        "[*] Sukces! Cała operacja HTML zakończona! Możesz sprawdzić pliki wynikowe.",
        "[-] Błąd podczas parsowania HTML: {:?}",
    );
}

```

## Plik-RustBinJob_03: `src/bin/job/pobieracz_geneteka_zakres_mapa__get.rs`

```rust
use fifak_lib::geneteka::parser_kml::parsuj_do_json;
use fifak_lib::geneteka::pobieracz_kml::pobierz_zakres;
use fifak_lib::logic::utils::{obsluz_wynik_parsowania, obsluz_wynik_pobierania};

#[tokio::main]
async fn main() {
    println!("[*] Uruchamiam narzędzia Geneteki (Moduł Mapy)...");

    // 1. Pobieranie KML (lub potwierdzenie istnienia z dzisiaj)
    let sciezka_kml = obsluz_wynik_pobierania(pobierz_zakres().await);

    println!("[+] Plik źródłowy KML gotowy do analizy: {}", sciezka_kml);

    // 2. Generujemy ścieżkę do pliku JSON (podmieniamy końcówkę .kml na .json)
    let sciezka_json = sciezka_kml.replace(".kml", ".json");

    // 3. Odpalamy nasz "Pożeracz XML-a"
    obsluz_wynik_parsowania(
        parsuj_do_json(&sciezka_kml, &sciezka_json),
        "[*] Sukces! Cała operacja zakończona!",
        "[-] Błąd podczas parsowania KML: {:?}",
    );
}

```

## Plik-RustBinTui_01: `src/bin/tui/app_kartezjan_produkt__tui.rs`

```rust
use fifak_lib::cli::menu::run_menu;

fn main() {
    // Żadnego sprawdzania argumentów, od razu wchodzimy w tryb interaktywny
    run_menu();
}

```

