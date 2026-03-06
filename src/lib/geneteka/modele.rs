use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rekord {
    pub lp: usize,
    pub miejsce: Miejsce,
    pub roczniki: Roczniki,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Miejsce {
    pub lonlat: [f64; 2],
    pub obszar: Vec<String>,
    pub parafia: Vec<String>,
}

// Nowa, lepsza nazwa!
#[derive(Debug, Serialize, Deserialize)]
pub struct Roczniki {
    pub u: Vec<u16>,
    pub m: Vec<u16>,
    pub z: Vec<u16>,
}

impl Roczniki {
    pub fn nowy() -> Self {
        Self {
            u: vec![],
            m: vec![],
            z: vec![],
        }
    }
}
