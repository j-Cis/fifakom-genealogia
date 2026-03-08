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