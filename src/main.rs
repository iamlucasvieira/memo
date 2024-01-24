use clap::{Parser, Subcommand};
use memo::app;
use memo::data;

const USERDATA: &str = "memo.txt";

#[derive(Parser)]
#[command(name = "memo")]
#[command(version = "0.1.0")]
#[command(about="A simple memo app", long_about=None)]
#[command(author = "Lucas Vieira dos Santos")]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists all memos
    List,
    Init,
}

fn main() {
    let cli = Cli::parse();
    let app_config = app::AppConfig::new("memo", USERDATA);

    if let Commands::Init = cli.command {
        match app::init(&app_config) {
            Ok(_) => println!(
                "Initialized data file at {}",
                app_config.data_file_path().display()
            ),
            Err(e) => eprintln!("Initialization error: {}", e),
        }
        return; // Exit after handling 'Init'
    }

    let mut memo_data = data::MemoData::new();
    if let Err(e) = data::DataFile::load(&mut memo_data, &app_config) {
        eprintln!(
            "Error loading data: {}\nHint: Use the `init` command to create a new file",
            e
        );
        return; // Optionally exit if the data file cannot be loaded
    }

    // Handle other commands
    let err = match cli.command {
        Commands::List => app::list(Box::new(memo_data)),
        // 'Init' is handled above, so it doesn't need to be here
        _ => unreachable!(), // This should not happen as all cases are covered
    };

    if let Err(e) = err {
        eprintln!("Error: {}", e);
    }
}
