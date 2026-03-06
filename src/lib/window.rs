// ./src/lib/window.rs

use i_slint_backend_winit::WinitWindowAccessor; // pozwala nam dobrać się do natywnego okna
use slint::{SharedString, Window};
use winit::window::ResizeDirection;

pub fn minimize(window: &Window) {
    window.set_minimized(true);
}

pub fn start_drag(window: &Window) {
    // Używamy "with_winit_window", aby uzyskać dostęp do funkcji systemowych
    window.with_winit_window(|winit_window| {
        // To wywołuje natywne przesuwanie okna systemu Windows/Linux/macOS
        // if let Err(e) = winit_window.drag_window() {
        //     eprintln!("Błąd podczas przesuwania okna: {}", e);
        // }
        // To odpalamy TYLKO dla myszki
        let _ = winit_window.drag_window();
    });
}

// Ręczne przesuwanie o zadaną wartość (dla dotyku)
pub fn window_move(window: &Window, delta_x: f32, delta_y: f32) {
    window.with_winit_window(|winit_window| {
        if let Ok(current_pos) = winit_window.outer_position() {
            let scale_factor = winit_window.scale_factor();
            // Przeliczamy logiczne piksele Slinta na fizyczne piksele ekranu
            let dx = (delta_x as f64 * scale_factor) as i32;
            let dy = (delta_y as f64 * scale_factor) as i32;

            let new_x = current_pos.x + dx;
            let new_y = current_pos.y + dy;

            winit_window.set_outer_position(winit::dpi::PhysicalPosition::new(new_x, new_y));
        }
    });
}

pub fn window_resize(window: &Window, direction: SharedString) {
    window.with_winit_window(|winit_window| {
        let dir = match direction.as_str() {
            "n" => ResizeDirection::North,
            "s" => ResizeDirection::South,
            "e" => ResizeDirection::East,
            "w" => ResizeDirection::West,
            "ne" => ResizeDirection::NorthEast,
            "nw" => ResizeDirection::NorthWest,
            "se" => ResizeDirection::SouthEast,
            "sw" => ResizeDirection::SouthWest,
            _ => return, // Nieznany kierunek - ignorujemy
        };
        let _ = winit_window.drag_resize_window(dir);

        if let Err(e) = winit_window.drag_resize_window(dir) {
            eprintln!("Resize error: {}", e);
        }
    });
}
