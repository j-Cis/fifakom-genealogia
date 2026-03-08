pub mod generator;
pub mod modele;
pub mod projekcje;
pub mod renderer;

// Dzięki temu w main.rs po prostu wpiszesz `use fifak_lib::atlas::generate_map_data;`
pub use generator::generate_map_data;
pub use modele::{MapProcessedData, MapProjection, NormalizedPoint};
pub use renderer::render_frame;
