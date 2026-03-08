use crate::geneteka::data_raw_modele::Rekord;

pub fn get_bounds(records: &[Rekord], margin: f64) -> (f64, f64, f64, f64) {
    let mut m_lon = f64::MAX;
    let mut mx_lon = f64::MIN;
    let mut m_lat = f64::MAX;
    let mut mx_lat = f64::MIN;

    for rek in records {
        if rek.miejsce.lonlat.len() == 2 {
            let lon = rek.miejsce.lonlat[0];
            let lat = rek.miejsce.lonlat[1];
            if lon < m_lon {
                m_lon = lon;
            }
            if lon > mx_lon {
                mx_lon = lon;
            }
            if lat < m_lat {
                m_lat = lat;
            }
            if lat > mx_lat {
                mx_lat = lat;
            }
        }
    }

    let lon_range = mx_lon - m_lon;
    let lat_range = mx_lat - m_lat;

    (
        m_lon - lon_range * margin,
        mx_lon + lon_range * margin,
        m_lat - lat_range * margin,
        mx_lat + lat_range * margin,
    )
}
