use serde::Deserialize;

// Główna struktura trzymająca całą tablicę [[rekord]]
#[derive(Debug, Deserialize)]
pub struct BazaGeneteki {
    pub rekord: Vec<Rekord>,
}

#[derive(Debug, Deserialize)]
pub struct Rekord {
    pub lp: u32,
    pub miejsce: Miejsce,
    pub roczniki: Roczniki,
}

#[derive(Debug, Deserialize)]
pub struct Miejsce {
    pub parafia: Vec<String>, // np. ["Baturyn", "6065"]
    pub obszar: Vec<String>,  // np. ["Białoruś", "22br"]
    pub lonlat: Vec<f64>,     // np. [27.860656, 54.054596]
}

#[derive(Debug, Deserialize)]
pub struct Roczniki {
    #[serde(default)]
    pub u: Vec<u32>,
    #[serde(default)]
    pub m: Vec<u32>,
    #[serde(default)]
    pub z: Vec<u32>,
}
