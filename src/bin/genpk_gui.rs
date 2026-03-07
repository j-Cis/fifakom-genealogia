#![windows_subsystem = "windows"]

use anyhow::Result;
use arboard::Clipboard;
use fifak_lib::logic::morphology::generate_morphology;
use fifak_lib::window;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

slint::include_modules!();

fn main() -> Result<()> {
    let ui= GenpkWindow::new()?; 
    let ctrl = ui.global::<WindowCtrl>();

    ctrl.on_window_quit(|| { let _ = slint::quit_event_loop(); });

    let ui_weak_min = ui.as_weak();
    ctrl.on_window_minimize(move || {
        if let Some(ui) = ui_weak_min.upgrade() { ui.window().set_minimized(true); }
    });
    
    let ui_weak_move = ui.as_weak();
    ctrl.on_window_move(move |dx, dy| {
        if let Some(ui) = ui_weak_move.upgrade() {
            if dx == 0.0 && dy == 0.0 {
                window::start_drag(ui.window());
            } else {
                window::window_move(ui.window(), dx, dy);
            }
        }
    });

    let ui_weak_resize = ui.as_weak();
    ctrl.on_window_resize(move |direction| {
        if let Some(ui) = ui_weak_resize.upgrade() {
            window::window_resize(ui.window(), direction);
        }
    });

    // 4. Generowanie (NAPRAWIONE: upgrade() zamiast unwrap())
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

    // 5. Schowek (NAPRAWIONE: upgrade() zamiast unwrap())
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