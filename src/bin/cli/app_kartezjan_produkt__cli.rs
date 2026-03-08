use clap::Parser;

use fifak_lib::logic::format::OutputFormat;
use fifak_lib::logic::format::format_result;
use fifak_lib::logic::morphology::generate_morphology;

#[derive(Parser)]
#[command(name = "Generator Morfologiczny - CLI")]
#[command(author = "Jaśko")]
#[command(version = "0.1.1")]
#[command(about = "Szybki generator wyrazów z wiersza poleceń")]
struct Cli {
    /// Wzór morfologiczny, np "(Zg|Sg|Zk|Sk)o(d|ds|dz)"
    #[arg(required = true)]
    pattern: String,

    /// Format wyjściowy
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Lista)]
    format: OutputFormat,
}

fn main() {
    // Od razu parsujemy i generujemy, bez zbędnych matchy
    let cli = Cli::parse();

    let names = generate_morphology(&cli.pattern);
    format_result(&names, &cli.format);
}
