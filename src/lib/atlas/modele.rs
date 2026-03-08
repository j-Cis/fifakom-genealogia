#[derive(Debug, Clone, Copy)]
pub enum MapProjection {
    ProjDynamic { margin: f64 },
    ProjWgs84,
    // W przyszłości:
    // Dymaxion,
    // AuthaGraph,
}

pub struct NormalizedPoint {
    pub x: f32,
    pub y: f32,
    pub name: String,
}

pub struct MapProcessedData {
    pub points: Vec<NormalizedPoint>,
    pub coastlines: Vec<Vec<(f32, f32)>>,
    pub min_lon: f64,
    pub max_lon: f64,
    pub min_lat: f64,
    pub max_lat: f64,
}
