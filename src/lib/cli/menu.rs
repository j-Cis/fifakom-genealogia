use crate::logic::format::OutputFormat;
use crate::logic::format::format_result;
use crate::logic::morphology::generate_morphology;
use inquire::{Select, Text};

pub fn run_menu() {
    let mut pattern = "(abc | def) ijk (1 | 2)".to_string();
    let mut output_format = OutputFormat::Lista; // Korzystamy z enuma!
    let mut running = true;

    // Przeniesione przed pętlę i użyte jako tablica (slice), nie wektor:
    let menu_options = [
        "Generuj",
        "Zmień wzór morfologiczny",
        "Zmień formę rezultatu",
        "Zmień sortowanie/grupowanie",
        "Ustawienia zaawansowane",
        "Wyjście",
    ];
    let fmt_options = ["lista", "przecinki", "markdown"];

    while running {
        println!("\n--- GENERATOR MORFOLOGICZNY ---");

        let choice = Select::new("Wybierz tryb:", menu_options.to_vec())
            .with_page_size(10)
            .prompt();

        match choice {
            Ok(opt) => match opt {
                "Generuj" => {
                    let names = generate_morphology(&pattern);
                    format_result(&names, &output_format);
                }
                "Zmień wzór morfologiczny" => {
                    let input = Text::new("Podaj wzór morfologiczny:")
                        .with_initial_value(&pattern)
                        .prompt()
                        .unwrap();
                    pattern = input.trim().to_string();
                }
                "Zmień formę rezultatu" => {
                    //let fmt_options = vec!["lista", "przecinki", "markdown"];
                    let fmt_choice = Select::new("Wybierz formę rezultatu:", fmt_options.to_vec())
                        .prompt()
                        .unwrap();

                    // Mapujemy wybór tekstowy na nasz bezpieczny Enum
                    output_format = match fmt_choice {
                        "przecinki" => OutputFormat::Przecinki,
                        "markdown" => OutputFormat::Markdown,
                        _ => OutputFormat::Lista,
                    };
                }
                "Zmień sortowanie/grupowanie" => {
                    println!("Opcje sortowania/grupowania na razie puste.");
                }
                "Ustawienia zaawansowane" => {
                    println!("Opcje zaawansowane na razie puste.");
                }
                "Wyjście" => {
                    running = false;
                }
                _ => {}
            },
            Err(_) => {
                println!("Błąd wyboru, wychodzimy.");
                running = false;
            }
        }
    }
}
