#![windows_subsystem = "windows"]

use anyhow::Result;
use arboard::Clipboard;
// DODANO: Model - bez tego metody row_count() i row_data() nie działają
use cpsgen_lib::logic::morphology::generate_morphology;
use cpsgen_lib::window;
use slint::{Model, ModelRc, SharedString, VecModel};

slint::include_modules!();

fn main() -> Result<()> {
    let app = AppWindow::new()?;

    // 1. Zamykanie aplikacji
    app.on_quit(move || {
        let _ = slint::quit_event_loop();
    });

    // 2. Przesuwanie okna (skoro działało, zostawiamy bez zmian)
    let app_weak_move = app.as_weak();
    app.on_move_window(move |dx, dy| {
        if let Some(ui) = app_weak_move.upgrade() {
            if dx == 0.0 && dy == 0.0 {
                window::start_drag(ui.window());
            } else {
                window::move_window(ui.window(), dx, dy);
            }
        }
    });

    // 3. Generowanie wyników
    let app_weak_send = app.as_weak();
    app.on_send(move |pattern| {
        let ui = app_weak_send.unwrap();
        let results = generate_morphology(pattern.as_str());

        // Aktualizacja widoku listy
        //let model: Vec<SharedString> = results.iter().map(|s| SharedString::from(s)).collect();
        let model: Vec<SharedString> = results.iter().map(SharedString::from).collect();
        ui.set_results(ModelRc::new(VecModel::from(model)));

        // Wysłanie połączonego tekstu (do widoku z przecinkami)
        let joined = results.join(", ");
        ui.set_joined_text(SharedString::from(joined));

        println!("Wygenerowano {} wyników", results.len());
    });

    // 4. Obsługa schowka (Kopiowanie)
    let app_weak_copy = app.as_weak();
    app.on_copy_to_clipboard(move |mode| {
        let ui = app_weak_copy.unwrap();

        let text_to_copy = if mode == "list" {
            let model = ui.get_results();
            // NAPRAWA: Dodano jawny typ Vec<String> dla kompilatora
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

        // Próba zapisu do schowka systemowego
        match Clipboard::new() {
            Ok(mut clipboard) => {
                if let Err(e) = clipboard.set_text(text_to_copy) {
                    eprintln!("Błąd schowka: {}", e);
                } else {
                    println!("Skopiowano do schowka!");
                }
            }
            Err(e) => eprintln!("Nie udało się zainicjować schowka: {}", e),
        }
    });

    app.run()?;
    Ok(())
}
