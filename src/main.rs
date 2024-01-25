use clap::Parser;
use memo::app;
use memo::data;

const USERDATA: &str = "memo.txt";

#[derive(Parser)]
#[command(name = "memo")]
#[command(version = "0.3.0")]
#[command(about="A simple memo app", long_about=None)]
#[command(author = "Lucas Vieira dos Santos")]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// Memo message
    message: Option<Vec<String>>,

    #[arg(short, long)]
    /// List all memos
    list: bool,

    #[arg(short, long)]
    /// Initialize the memo file
    init: bool,

    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..))]
    /// Remove a memo by ID
    remove: Option<u32>,
}

fn main() {
    let cli = Cli::parse();
    let app_config = app::AppConfig::new("memo", USERDATA);

    // Handle 'Init' command
    if cli.init {
        match app::init(&app_config) {
            Ok(_) => println!(
                "Initialized data file at {}",
                app_config.data_file_path().display()
            ),
            Err(e) => eprintln!("Initialization error: {}", e),
        }
    }

    let mut memo_data = data::MemoData::new();
    if let Err(e) = data::DataFile::load(&mut memo_data, &app_config) {
        eprintln!(
            "Error loading data: {}\nHint: Use the `-i` flag to initialize the data file",
            e
        );
        return; // Optionally exit if the data file cannot be loaded
    }

    // Handle message
    let has_message = cli.message.is_some();
    if let Some(message) = cli.message {
        if let Err(e) = app::add(&mut memo_data, &app_config, message.join(" ")) {
            eprintln!("Error: {}", e);
        }
    }

    // Handle remove
    if let Some(id) = cli.remove {
        if let Err(e) = app::remove(&mut memo_data, &app_config, id) {
            eprintln!("Error: {}", e);
        }
    }

    // Handle list
    if cli.list || !has_message {
        if let Err(e) = app::list(&memo_data) {
            eprintln!("Error: {}", e);
        }
    }
}
