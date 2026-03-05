use clap::{Parser, Subcommand};

use cpsgen_lib::logic::format::OutputFormat;
use cpsgen_lib::logic::format::format_result;
use cpsgen_lib::logic::morphology::generate_morphology;
use cpsgen_lib::cli::menu::run_menu;

/// CLI i menu interaktywne w jednym pliku
#[derive(Parser)]
#[command(name = "Generator Morfologiczny Wyrazów - CLI i Menu")]
#[command(author = "Jaśko")]
#[command(version = "0.1.1")]
#[command(about = "Generuje wyrazy według wzoru morfologicznego")]
struct Cli {
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
	/// Generuj wyrazy według wzoru
	Generate {
		/// Wzór morfologiczny, np "(Zg|Sg|Zk|Sk)o(d|ds|dz)"
		pattern: String,

		/// Format wyjściowy (lista, przecinki, markdown)
		#[arg(short, long, value_enum, default_value_t = OutputFormat::Lista)]
		format: OutputFormat,
	},
	/// Wyjdź z programu
	Exit,
}

fn main() {
	let cli = Cli::parse();

	match &cli.command {
		// Tryb CLI → generujemy od razu
		Some(Commands::Generate { pattern, format }) => {
			let names = generate_morphology(pattern);
			format_result(&names, format); // domyślnie lista
		}
		// Inne przypadki → menu interaktywne po dwukliku .exe
		_ => run_menu(),
	}
}
