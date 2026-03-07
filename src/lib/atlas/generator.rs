use crate::geneteka::data_raw_modele::Rekord;
use super::modele::{MapProcessedData, MapProjection, NormalizedPoint};
use super::projekcje::{proj_dynamiczna, proj_plate_carree};

pub fn generate_map_data(records: &[Rekord], projection: MapProjection) -> MapProcessedData {
    // 1. Ustalenie granic przez wybrany silnik projekcji
    let (min_lon, max_lon, min_lat, max_lat) = match projection {
        MapProjection::ProjWgs84 => proj_plate_carree::get_bounds(),
        MapProjection::ProjDynamic { margin } => proj_dynamiczna::get_bounds(records, margin),
    };

    let lon_range = max_lon - min_lon;
    let lat_range = max_lat - min_lat;

    // 2. Transformacja współrzędnych
    let mut points = Vec::new();
    for rek in records {
        if rek.miejsce.lonlat.len() == 2 {
            let lon = rek.miejsce.lonlat[0];
            let lat = rek.miejsce.lonlat[1];
            let name = rek.miejsce.parafia.first().cloned().unwrap_or_default();

            // Rzutowanie liniowe (podstawa Plate Carree)
            let x_norm = if lon_range > 0.0 { (lon - min_lon) / lon_range } else { 0.5 };
            let y_norm = if lat_range > 0.0 { (lat - min_lat) / lat_range } else { 0.5 };

            // Odrzucamy błędne punkty, które lądują w kosmosie
            if (0.0..=1.0).contains(&x_norm) && (0.0..=1.0).contains(&y_norm) {
                points.push(NormalizedPoint {
                    x: x_norm as f32,
                    y: (1.0 - y_norm) as f32, // Odwrócona oś Y w grafice 2D
                    name,
                });
            }
        }
    }

    MapProcessedData {
        points,
        min_lon,
        max_lon,
        min_lat,
        max_lat,
    }
}