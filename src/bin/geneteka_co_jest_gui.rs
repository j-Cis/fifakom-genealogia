#![windows_subsystem = "windows"]

use anyhow::Result;
use fifak_lib::window; 
use fifak_lib::setup_window_ctrl_bindings;
use slint::ComponentHandle;

slint::include_modules!();

fn main() -> Result<()> {
    let ui= GenetekaCoJestWindow::new()?;
    setup_window_ctrl_bindings!(ui);

    
    ui.on_search(|text| {
        println!("[*] Szukamy: {}", text);
    });

    ui.run()?;
    Ok(())
}