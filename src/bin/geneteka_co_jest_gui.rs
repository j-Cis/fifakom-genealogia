#![windows_subsystem = "windows"]

use anyhow::Result;
use fifak_lib::window; 
use slint::ComponentHandle;

slint::include_modules!();

fn main() -> Result<()> {
    let ui= GenetekaCoJestWindow::new()?;
    let ctrl = ui.global::<WindowCtrl>();ctrl.on_window_quit(|| { let _ = slint::quit_event_loop(); });

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

    // 4. Szukanie (na razie puste)
    ui.on_search(|text| {
        println!("[*] Szukamy: {}", text);
    });

    ui.run()?;
    Ok(())
}