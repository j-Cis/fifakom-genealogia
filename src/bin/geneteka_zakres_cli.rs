use cpsgen_lib::geneteka::parser_kml::parsuj_do_json;
use cpsgen_lib::geneteka::zakres_googlemaps::pobierz_zakres;

#[tokio::main]
async fn main() {
    println!("[*] Uruchamiam narzędzia Geneteki...");

    // 1. Pobieranie KML (lub potwierdzenie istnienia z dzisiaj)
    let sciezka_kml = match pobierz_zakres().await {
        Ok(sciezka) => sciezka,
        Err(blad) => {
            eprintln!("[-] Niestety, pobieranie zawiodło: {:?}", blad);
            return;
        }
    };

    println!("[+] Dane źródłowe zabezpieczone: {}", sciezka_kml);

    // 2. Generujemy ścieżkę do pliku JSON (podmieniamy końcówkę .kml na .json)
    let sciezka_json = sciezka_kml.replace(".kml", ".json");

    // 3. Odpalamy nasz "Pożeracz XML-a"
    match parsuj_do_json(&sciezka_kml, &sciezka_json) {
        Ok(_) => println!("[*] Cała operacja zakończona pełnym sukcesem!"),
        Err(e) => eprintln!("[-] Błąd podczas parsowania danych: {:?}", e),
    }
}
