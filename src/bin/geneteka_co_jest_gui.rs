#![windows_subsystem = "windows"]

use anyhow::Result;

use std::rc::Rc;
use slint::ComponentHandle; // UWAGA: Usunąłem stąd VecModel, bo już go nie używamy!

use fifak_lib::window; 
use fifak_lib::setup_window_ctrl_bindings;
use fifak_lib::atlas::{generate_map_data, MapProjection};

slint::include_modules!();

fn main() -> Result<()> {
    let sciezka_danych = "./data/genealodzy-geneteka/raw";

    let baza = fifak_lib::geneteka::pobrany_najnowszy::laduj_baze(sciezka_danych)
        .unwrap_or_else(|| {
            println!("Nie znaleziono danych w {}. Tworzę pustą mapę.", sciezka_danych);
            fifak_lib::geneteka::data_raw_modele::BazaGeneteki { rekord: vec![] }
        });

    println!("Wczytano {} parafii!", baza.rekord.len());

    let ui= GenetekaCoJestWindow::new()?;
    setup_window_ctrl_bindings!(ui);

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
                rot
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