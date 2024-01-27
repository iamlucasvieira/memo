use anyhow::Result;
use clap::Parser;
use memo::app;
use memo::data;
use memo::models;

mod commands;

const USERDATA: &str = "memo.txt";

#[derive(Parser)]
#[command(name = "memo")]
#[command(version = "0.4.0")]
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

    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..), num_args=1..)]
    /// Remove a memo by ID
    remove: Option<Vec<u32>>,
}

fn main() {
    let cli = Cli::parse();
    let app_config = app::AppConfig::new("memo", USERDATA);

    // Handle 'Init' command
    if cli.init {
        let _ = display_result(
            commands::init(&app_config),
            Some("Initialized data file"),
            Some("Initialization error"),
        );
        return;
    }

    let mut memo_data = models::MemoData::new();

    if display_result(
        data::DataFile::load(&mut memo_data, &app_config),
        None,
        Some("Could not load data file"),
    )
    .is_err()
    {
        return; //  exit if the data file cannot be loaded
    }

    // Handle message
    let has_message = cli.message.is_some();

    if let Some(message) = cli.message {
        let _ = display_result(
            commands::add(&mut memo_data, &app_config, message.join(" ")),
            None,
            Some("Could not add memo"),
        );
    }

    // Handle remove
    if let Some(id) = cli.remove {
        let _ = display_result(
            commands::remove(&mut memo_data, &app_config, id),
            None,
            Some("Could not remove memo"),
        );
    }

    // Handle list
    if cli.list || !has_message {
        let _ = display_result(
            commands::list(&memo_data),
            None,
            Some("Could not list memos"),
        );
    }
}

/// Prints  restult or error to stderror if error found. Option ok and err messages can be customized.
fn display_result<T>(
    result: Result<T>,
    ok_message: Option<&str>,
    err_message: Option<&str>,
) -> Result<T> {
    match (&result, ok_message, err_message) {
        (Ok(_), Some(ok), _) => eprintln!("{}", ok),
        (Err(err), _, Some(err_msg)) => eprintln!("{}: {}", err_msg, err),
        (Err(err), _, None) => eprintln!("{}", err),
        _ => (),
    }
    result
}
