use tiny_skia::{Color, Paint, Pixmap, Rect, Transform};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use super::modele::MapProcessedData;

pub fn render_frame(
    width: u32,
    height: u32,
    map_data: &MapProcessedData,
    offset_x: f32,
    offset_y: f32,
    zoom: f32,
) -> Image {
    // Slint może na starcie podać wymiar 0x0 zanim ułoży okno
    if width == 0 || height == 0 {
        return Image::default();
    }

    // 1. Alokacja płótna w RAM
    let mut pixmap = Pixmap::new(width, height).unwrap();
    pixmap.fill(Color::from_rgba8(35, 35, 40, 255)); // Ciemnografitowe tło oceanów

    // 2. Pędzel do kropek
    let mut paint = Paint::default();
    paint.set_color_rgba8(255, 0, 85, 255); // Genetykowy czerwony róż!
    paint.anti_alias = true;

    let world_w = width as f32 * zoom;
    let world_h = height as f32 * zoom;

    // 3. SPATIAL CULLING - iterujemy po punktach, ale rysujemy tylko te widoczne
    for pt in &map_data.points {
        let screen_x = (pt.x * world_w) + offset_x;
        let screen_y = (pt.y * world_h) + offset_y;

        // Odrzucamy wszystko, co wyleciało poza nasz ekran!
        if screen_x >= -5.0 && screen_x <= width as f32 + 5.0 &&
           screen_y >= -5.0 && screen_y <= height as f32 + 5.0 {
            
            if let Some(rect) = Rect::from_xywh(screen_x - 2.0, screen_y - 2.0, 4.0, 4.0) {
                pixmap.fill_rect(rect, &paint, Transform::identity(), None);
            }
        }
    }

    // 4. Błyskawiczna konwersja surowych pikseli do formatu Slinta
    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        pixmap.data(),
        width,
        height,
    );
    Image::from_rgba8(buffer)
}