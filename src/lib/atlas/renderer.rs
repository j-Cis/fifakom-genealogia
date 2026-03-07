use tiny_skia::{Color, Paint, Pixmap, Rect, Transform, PathBuilder, Stroke};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use super::modele::MapProcessedData;

pub fn render_frame(
    width: u32,
    height: u32,
    map_data: &MapProcessedData,
    offset_x: f32,
    offset_y: f32,
    zoom: f32,
    rotation_deg: f32,
) -> Image {
    if width == 0 || height == 0 { return Image::default(); }

    let mut pixmap = Pixmap::new(width, height).unwrap();
    // Tło - ustawiam na bardzo ciemny szary, żeby lądy były widoczne
    pixmap.fill(Color::from_rgba8(30, 30, 35, 255));

    let rot_rad = rotation_deg.to_radians();
    let cos_a = rot_rad.cos();
    let sin_a = rot_rad.sin();
    
    let world_w = width as f32 * zoom;
    let world_h = height as f32 * zoom;
    let pivot_x = world_w / 2.0;
    let pivot_y = world_h / 2.0;

    // --- 1. RYSOWANIE LINII BRZEGOWYCH (MAPA BAZOWA) ---
    let mut paint_map = Paint::default();
    // Ustawiam JASKRAWY ZIELONY na moment testu, żebyś widział czy w ogóle coś rysuje
    paint_map.set_color_rgba8(0, 255, 100, 255); 
    paint_map.anti_alias = true;
    let stroke = Stroke { width: 1.0, ..Default::default() };

    for linia in &map_data.coastlines {
        if linia.is_empty() { continue; }
        let mut pb = PathBuilder::new();
        
        for (i, p) in linia.iter().enumerate() {
            // MATEMATYKA IDENTYCZNA JAK DLA KROPEK
            let local_x = p.0 * world_w;
            let local_y = p.1 * world_h;
            let dx = local_x - pivot_x;
            let dy = local_y - pivot_y;
            let rot_x = dx * cos_a - dy * sin_a;
            let rot_y = dx * sin_a + dy * cos_a;
            let sx = pivot_x + rot_x + offset_x;
            let sy = pivot_y + rot_y + offset_y;

            if i == 0 { pb.move_to(sx, sy); } else { pb.line_to(sx, sy); }
        }
        
        if let Some(path) = pb.finish() {
            pixmap.stroke_path(&path, &paint_map, &stroke, Transform::identity(), None);
        }
    }

    // --- 2. RYSOWANIE PUNKTÓW (GENETEKA) ---
    let mut paint_pt = Paint::default();
    paint_pt.set_color_rgba8(255, 0, 85, 255);
    paint_pt.anti_alias = true;

    for pt in &map_data.points {
        let local_x = pt.x * world_w;
        let local_y = pt.y * world_h;
        let dx = local_x - pivot_x;
        let dy = local_y - pivot_y;
        let rot_x = dx * cos_a - dy * sin_a;
        let rot_y = dx * sin_a + dy * cos_a;
        let sx = pivot_x + rot_x + offset_x;
        let sy = pivot_y + rot_y + offset_y;
        
        if sx >= -5.0 && sx <= width as f32 + 5.0 && sy >= -5.0 && sy <= height as f32 + 5.0 {
            if let Some(rect) = Rect::from_xywh(sx - 2.0, sy - 2.0, 4.0, 4.0) {
                pixmap.fill_rect(rect, &paint_pt, Transform::identity(), None);
            }
        }
    }

    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(pixmap.data(), width, height);
    Image::from_rgba8(buffer)
}