use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)] // ValueEnum pozwala wpiąć to od razu w CLI (clap)
pub enum OutputFormat {
    Lista,
    Przecinki,
    Markdown,
}

/// Wyświetlanie wyników w wybranym formacie
pub fn format_result(names: &[String], format: &OutputFormat) {
    match format {
        OutputFormat::Lista => {
            for n in names {
                println!("- {}", n);
            }
        }
        OutputFormat::Przecinki => println!("{}", names.join(", ")),
        OutputFormat::Markdown => {
            println!("| Wyraz |\n| :--- |");
            for n in names {
                println!("| {} |", n);
            }
        }
    }
}
