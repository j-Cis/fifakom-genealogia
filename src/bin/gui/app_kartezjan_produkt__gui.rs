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
