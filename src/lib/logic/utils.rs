use anyhow::Result;
use std::process;

/// Funkcja pomocnicza, która wypakowuje ścieżkę z wyniku lub elegancko kończy program w razie błędu.
pub fn obsluz_wynik_pobierania(wynik: Result<String>) -> String {
    match wynik {
        Ok(sciezka) => sciezka,
        Err(blad) => {
            eprintln!("[-] Niestety, operacja zawiodła: {:?}", blad);
            // Zamykamy program z kodem błędu 1 (standard w systemach operacyjnych dla błędu)
            process::exit(1);
        }
    }
}

/// Funkcja pomocnicza do obsługi wyników parsowania (nie zwraca danych, tylko komunikaty)
pub fn obsluz_wynik_parsowania(wynik: Result<()>, msg_sukces: &str, msg_blad: &str) {
    match wynik {
        Ok(_) => println!("[*] {}", msg_sukces),
        Err(e) => {
            eprintln!("[-] {}: {:?}", msg_blad, e);
            process::exit(1);
        }
    }
}
