#![windows_subsystem = "windows"]

use anyhow::Result;

use std::rc::Rc;
use slint::{ComponentHandle,  VecModel};

use fifak_lib::window; 
use fifak_lib::setup_window_ctrl_bindings;


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

    // --- LOGIKA MAPY ---
    // Pobieramy rozmiar płótna ze Slinta (ustawiłeś domyślnie na 700x600)
    // let canvas_w = ui.get_map_canvas_width();
    // let canvas_h = ui.get_map_canvas_height();

    // Szukamy skrajnych punktów (bounding box) żeby mapa idealnie się wpasowała
    let mut min_lon = f64::MAX;
    let mut max_lon = f64::MIN;
    let mut min_lat = f64::MAX;
    let mut max_lat = f64::MIN;

    for rek in &baza.rekord {
        if rek.miejsce.lonlat.len() == 2 {
            let lon = rek.miejsce.lonlat[0];
            let lat = rek.miejsce.lonlat[1];
            if lon < min_lon { min_lon = lon; }
            if lon > max_lon { max_lon = lon; }
            if lat < min_lat { min_lat = lat; }
            if lat > max_lat { max_lat = lat; }
        }
    }

    // Margines, żeby kropki nie wchodziły na obramowanie
    let margin = 0.05; 
    let lon_range = (max_lon - min_lon) * (1.0 + 2.0 * margin);
    let lat_range = (max_lat - min_lat) * (1.0 + 2.0 * margin);
    let lon_offset = min_lon - (max_lon - min_lon) * margin;
    let lat_offset = min_lat - (max_lat - min_lat) * margin;

    // Przeliczamy współrzędne każdego punktu na piksele
    let mut punkty = Vec::new();
    for rek in &baza.rekord {
        if rek.miejsce.lonlat.len() == 2 {
            let lon = rek.miejsce.lonlat[0];
            let lat = rek.miejsce.lonlat[1];
            let nazwa = rek.miejsce.parafia.first().cloned().unwrap_or_default();

            let x_norm = if lon_range > 0.0 { (lon - lon_offset) / lon_range } else { 0.5 };
            let y_norm = if lat_range > 0.0 { (lat - lat_offset) / lat_range } else { 0.5 };

            punkty.push(MapPoint {
                x: x_norm as f32,
                y: (1.0 - y_norm) as f32, // Odwrócona oś Y
                nazwa: nazwa.into(),
            });
        }
    }

    // Wypychamy wyliczone punkty prosto na front-end do Slinta
    let points_model = Rc::new(VecModel::from(punkty));
    ui.set_map_points(points_model.into());
    // -------------------
    
    ui.on_search(|text| {
        println!("[*] Szukamy: {}", text);
    });

    ui.run()?;
    Ok(())
}