fn main() {
    // Ścieżka musi dokładnie wskazywać na Twój główny plik .slint
    slint_build::compile("src/ui/app-window.slint").unwrap();
}
