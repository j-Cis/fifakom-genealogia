# [fifakom-genealogia](https://github.com/j-Cis/fifakom-genealogia/releases) **„Genealogia dla sprytnych”**

Zestaw narzędzi i sztuczek pomocnych genealogom.

| № | Nazwa GUI | Nazwa CLI | OS | Cel |
| --- | :--- | :--- | :---: | :--- |
| 1. | GenProdKart | gen_pk | Win(x64) | **Iloczyn Kartezjański/Generator Kombinacji** Narzędzie do generowania kombinacji tekstowych na podstawie wzorców wykorzystujących iloczyn kartezjański. <br> Idealne do genrowania wariantów nazwisk. `(Zg \| Sg \| Zk \| Sk)  o (d \| ds \| dz \| c \| cz) (a \| ai \| ay \| aÿ \| aj \| aij \| e \| ei \| ey \| eÿ \| ej \| eij)` |

Gotowe 📦 pliki `.exe` znajdą się w folderze `./dist/win/release/`.

## NARZĘDZIA

### CPSGen 

## 🛠 Samodzielna kompilacja (do korekty - nie aktyualne)

Projekt został napisany w języku Rust przy użyciu frameworka Slint.

Aby zbudować obie wersje (CLI i GUI) w trybie optymalizowanym:

```bash
cargo make release

```

### Wymagania

- [Rust](https://www.rust-lang.org/tools/install) (wersja stabilna)
- `cargo-make` (opcjonalnie, do automatyzacji zadań):

  ```bash
  cargo install cargo-make

  ```



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


## ⚖ Licencja

Projekt stworzony na własny użytek. Wolno kopiować i modyfikować.
