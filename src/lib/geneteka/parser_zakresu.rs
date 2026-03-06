/// Funkcja, która tłumaczy "1846,61-62" na czystą listę lat: [1846, 1861, 1862]
pub fn rozkoduj_lata(dane: &str) -> Vec<u16> {
    let mut wyniki = Vec::new();
    let mut ostatnie_stulecie = 1800; // Baza, na wypadek gdyby pierwsza liczba była 2-cyfrowa

    for czesc in dane.split(',') {
        let czesc = czesc.trim();
        if czesc.is_empty() {
            continue;
        }

        if czesc.contains('-') {
            // Mamy zakres np. 1846-1850 lub 61-62
            let granice: Vec<&str> = czesc.split('-').collect();
            if granice.len() == 2 {
                let start = parsuj_rok(granice[0], &mut ostatnie_stulecie);
                let koniec = parsuj_rok(granice[1], &mut ostatnie_stulecie);
                if start > 0 && koniec >= start {
                    wyniki.extend(start..=koniec);
                }
            }
        } else {
            // Pojedynczy rok np. 1846 lub 61
            let rok = parsuj_rok(czesc, &mut ostatnie_stulecie);
            if rok > 0 {
                wyniki.push(rok);
            }
        }
    }

    // Sortowanie i usuwanie duplikatów (uniqueBy + sort z Twojego JS-a)
    wyniki.sort_unstable();
    wyniki.dedup();
    wyniki
}

/// Rozpoznaje czy rok jest 4-cyfrowy (1846) czy 2-cyfrowy (61)
fn parsuj_rok(tekst: &str, ostatnie_stulecie: &mut u16) -> u16 {
    let tekst = tekst.trim();
    if tekst.len() == 4 {
        if let Ok(rok) = tekst.parse::<u16>() {
            *ostatnie_stulecie = (rok / 100) * 100; // Zapamiętuje np. 1800 z 1846
            return rok;
        }
    } else if tekst.len() == 2
        && let Ok(skrot) = tekst.parse::<u16>()
    {
        return *ostatnie_stulecie + skrot; // np. 1800 + 61 = 1861
    }
    0 // Błąd parsowania
}
