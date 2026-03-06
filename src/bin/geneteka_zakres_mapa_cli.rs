use fifak_lib::geneteka::pobieracz_kml::pobierz_zakres;
use fifak_lib::geneteka::parser_kml::parsuj_do_json;
use fifak_lib::logic::utils::{obsluz_wynik_parsowania, obsluz_wynik_pobierania};

#[tokio::main]
async fn main() {
    println!("[*] Uruchamiam narzędzia Geneteki (Moduł Mapy)...");

    // 1. Pobieranie KML (lub potwierdzenie istnienia z dzisiaj)
    let sciezka_kml = obsluz_wynik_pobierania(pobierz_zakres().await);

    println!("[+] Plik źródłowy KML gotowy do analizy: {}", sciezka_kml);

    // 2. Generujemy ścieżkę do pliku JSON (podmieniamy końcówkę .kml na .json)
    let sciezka_json = sciezka_kml.replace(".kml", ".json");

    // 3. Odpalamy nasz "Pożeracz XML-a"
    obsluz_wynik_parsowania(
        parsuj_do_json(&sciezka_kml, &sciezka_json),
        "[*] Sukces! Cała operacja zakończona!",
        "[-] Błąd podczas parsowania KML: {:?}"
    );
}
