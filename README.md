# CPSGen (Cartesian Product Segment Generator)

Narzędzie do generowania kombinacji tekstowych na podstawie wzorców wykorzystujących iloczyn kartezjański. Idealne do tworzenia list słów, testowania morfologii lub generowania wariantów haseł/nazw.

## 🚀 Funkcje

- **Dwa tryby pracy**: Szybka konsola (CLI) oraz nowoczesny interfejs graficzny (GUI).
- **Zaawansowane wzorce**: Obsługa opcji typu `(a|b|c)` do generowania wszystkich możliwych kombinacji.
- **Tryby widoku (GUI)**: Możliwość wyświetlania wyników jako listy lub ciągłego tekstu.
- **Kopiowanie do schowka**: Szybki eksport wyników jednym kliknięciem.
- **Tryb Jasny/Ciemny**: Automatyczne dostosowanie do preferencji systemowych lub ręczna zmiana.
- **Brak okna konsoli**: Wersja GUI działa jako czysta aplikacja Windows.

## 🛠 Instalacja i Budowanie

Projekt został napisany w języku Rust przy użyciu frameworka Slint.

### Wymagania

- [Rust](https://www.rust-lang.org/tools/install) (wersja stabilna)
- `cargo-make` (opcjonalnie, do automatyzacji zadań):

  ```bash
  cargo install cargo-make

  ```

### Kompilacja

Aby zbudować obie wersje (CLI i GUI) w trybie optymalizowanym:

```bash
cargo make release

```

Gotowe pliki `.exe` znajdą się w folderze `dist/win/release/`.

## 📂 Struktura dystrybucji (dist)

Po wykonaniu powyższej komendy, gotowe i zoptymalizowane pliki binarne zostaną automatycznie skopiowane do folderu dist/win/release/. Znajdziesz tam:

- CPSGenGUI.exe – Pełna wersja graficzna z obsługą motywów i schowka.

- CPSGen.exe – Lekka wersja konsolowa (CLI) do szybkich operacji w terminalu.

## 📖 Instrukcja Obsługi

### Wzorce (Patterny)

Aplikacja rozpoznaje nawiasy oraz pionową kreskę jako separator opcji.

- **Przykład**: `Cis (o|w)`
- **Wynik**:
- `Ciso`
- `Cisw`

### Wersja GUI (`CPSGenGUI.exe`)

1. Wpisz wzór w pole **Wzór...**.
2. Kliknij **GENERUJ** lub naciśnij Enter.
3. Wybierz sposób wyświetlania (**LISTA** lub **TEKST**).
4. Kliknij **KOPIUJ WYNIKI**, aby przenieść dane do innego programu.

### Wersja CLI (`CPSGen.exe`)

Uruchom program z terminala, podając wzór jako argument:

```bash
./CPSGen.exe "Cis (o|w)"

```

## 📂 Struktura Projektu

- `src/lib.rs` - Rdzeń logiczny generatora.
- `src/bin/gui.rs` - Kod źródłowy aplikacji graficznej.
- `src/bin/main.rs` - Kod źródłowy narzędzia konsolowego.
- `src/ui/` - Pliki interfejsu użytkownika `.slint` oraz style.
- `Makefile.toml` - Konfiguracja automatyzacji budowania i dystrybucji.

## ⚖ Licencja

Projekt stworzony na własny użytek. Wolno kopiować i modyfikować.
