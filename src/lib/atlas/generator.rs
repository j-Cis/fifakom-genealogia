use std::fs;
use geojson::{GeoJson, Value};
use crate::geneteka::data_raw_modele::Rekord;
use super::modele::{MapProcessedData, MapProjection, NormalizedPoint};
use super::projekcje::{proj_dynamiczna, proj_plate_carree};
use crate::pliki::path_data_naturalearth as paths;

pub fn generate_map_data(records: &[Rekord], projection: MapProjection) -> MapProcessedData {
    let (min_lon, max_lon, min_lat, max_lat) = match projection {
        MapProjection::ProjWgs84 => proj_plate_carree::get_bounds(),
        MapProjection::ProjDynamic { margin } => proj_dynamiczna::get_bounds(records, margin),
    };

    let lon_range = max_lon - min_lon;
    let lat_range = max_lat - min_lat;

    let mut coastlines = Vec::new();
    
    // FAST-FALL: Używamy Twoich dokładnych nazw z rozszerzeniem .geojson
    let sciezki_geo = [
        paths::PATH_COASTLINE_110M,
        paths::PATH_GLACIERS_110M,
        paths::PATH_LAKES_110M,
        paths::PATH_RIVERS_LAKE_CENTER_50M,
        paths::PATH_RAMKA,
    ];
    
    for sciezka in sciezki_geo {
        // .expect spowoduje CRASH i poda ścieżkę, jeśli pliku nie ma pod tym adresem
        let data = fs::read_to_string(sciezka)
            .expect(&format!("KRYTYCZNY BŁĄD: Nie znaleziono pliku: {}", sciezka));
            
        let res: GeoJson = data.parse()
            .expect(&format!("KRYTYCZNY BŁĄD: Plik {} nie jest poprawnym GeoJSONem", sciezka));
            
        wyciagnij_linie(&res, &mut coastlines, min_lon, max_lon, min_lat, max_lat);
    }

    // Diagnostyka w konsoli
    println!(">>> GENERATOR: Wczytano {} segmentów mapy bazowej", coastlines.len());

    let mut points = Vec::new();
    for rek in records {
        if rek.miejsce.lonlat.len() == 2 {
            let lon = rek.miejsce.lonlat[0];
            let lat = rek.miejsce.lonlat[1];
            let name = rek.miejsce.parafia.first().cloned().unwrap_or_default();

            let x_norm = (lon - min_lon) / lon_range;
            let y_norm = (lat - min_lat) / lat_range;

            if (0.0..=1.0).contains(&x_norm) && (0.0..=1.0).contains(&y_norm) {
                points.push(NormalizedPoint {
                    x: x_norm as f32,
                    y: (1.0 - y_norm) as f32,
                    name,
                });
            }
        }
    }

    MapProcessedData {
        points,
        coastlines,
        min_lon, max_lon, min_lat, max_lat,
    }
}

fn wyciagnij_linie(gj: &GeoJson, out: &mut Vec<Vec<(f32, f32)>>, min_lon: f64, max_lon: f64, min_lat: f64, max_lat: f64) {
    let lon_range = max_lon - min_lon;
    let lat_range = max_lat - min_lat;

    if let GeoJson::FeatureCollection(fc) = gj {
        for feature in &fc.features {
            if let Some(geom) = &feature.geometry {
                match &geom.value {
                    Value::LineString(ls) => {
                        let coords: Vec<(f32, f32)> = ls.iter().map(|p| {
                            let x = (p[0] - min_lon) / lon_range;
                            let y = (max_lat - p[1]) / lat_range;
                            (x as f32, y as f32)
                        }).collect();
                        out.push(coords);
                    }
                    Value::MultiLineString(mls) => {
                        for ls in mls {
                            let coords: Vec<(f32, f32)> = ls.iter().map(|p| {
                                let x = (p[0] - min_lon) / lon_range;
                                let y = (max_lat - p[1]) / lat_range;
                                (x as f32, y as f32)
                            }).collect();
                            out.push(coords);
                        }
                    }
                    Value::Polygon(p) => {
                        for ring in p {
                            let coords: Vec<(f32, f32)> = ring.iter().map(|p| {
                                let x = (p[0] - min_lon) / lon_range;
                                let y = (max_lat - p[1]) / lat_range;
                                (x as f32, y as f32)
                            }).collect();
                            out.push(coords);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}