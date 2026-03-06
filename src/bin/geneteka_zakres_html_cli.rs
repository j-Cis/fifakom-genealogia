use fifak_lib::geneteka::pobieracz_html::pobierz_rejestry;
use fifak_lib::geneteka::parser_html::parsuj_html;
use fifak_lib::logic::utils::{obsluz_wynik_parsowania, obsluz_wynik_pobierania};

#[tokio::main]
async fn main() {
    println!("[*] Uruchamiam narzędzia Geneteki (Moduł Rejestrów HTML)...");

    // 1. Pobieranie lub wczytywanie pliku HTML
    let sciezka_html = obsluz_wynik_pobierania(pobierz_rejestry().await);

    println!("[+] Plik źródłowy HTML gotowy do analizy: {}", sciezka_html);

    // 2. Przygotowanie ścieżki docelowej
    // Wynik zapiszemy jako rejestry_YYYY-MM-DD.json
    let sciezka_json = sciezka_html.replace(".html", ".json");

    // 3. Odpalamy nasz "Kombajn do HTML-a"
    obsluz_wynik_parsowania(
        parsuj_html(&sciezka_html, &sciezka_json),
        "[*] Sukces! Cała operacja HTML zakończona! Możesz sprawdzić pliki wynikowe.",
        "[-] Błąd podczas parsowania HTML: {:?}"
    );
}
    