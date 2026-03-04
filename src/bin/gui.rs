use anyhow::Result;
use slint::{ModelRc, SharedString, VecModel};
use cpsgen_lib::logic::morphology::generate_morphology;
use cpsgen_lib::window;

slint::include_modules!();

fn main() -> Result<()> {
    let app = AppWindow::new()?;

    app.on_quit(move || {
        let _ = slint::quit_event_loop();
    });

    // Skoro to działało, zostawiamy bez zmian
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

    let app_weak_send = app.as_weak();
    app.on_send(move |pattern| {
        let ui = app_weak_send.unwrap();
        let results = generate_morphology(pattern.as_str());
        let model: Vec<SharedString> = results.into_iter().map(SharedString::from).collect();
        ui.set_results(ModelRc::new(VecModel::from(model)));
    });

    app.run()?;
    Ok(())
}