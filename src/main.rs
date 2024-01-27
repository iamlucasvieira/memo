use anyhow::Result;
use clap::Parser;
use memo::app;
use memo::data;
use memo::models;
use memo::style;

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
    /// List memos grouped by date
    list: bool,

    #[arg(short, long)]
    /// Initialize the memo file
    init: bool,

    #[arg(short, long)]
    /// List memos sorted by ID
    sorted: bool,

    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..), num_args=1..)]
    /// Remove a memo by ID
    remove: Option<Vec<u32>>,
}

fn main() {
    let cli = Cli::parse();
    let app_config = app::AppConfig::new("memo", USERDATA);
    let has_no_flags = !cli.list && !cli.init && cli.message.is_none() && cli.remove.is_none();

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
    if cli.list || has_no_flags {
        let mode = if cli.sorted {
            data::DisplayMode::Sorted
        } else {
            data::DisplayMode::GroupByDate
        };

        let _ = display_result(
            commands::list(&memo_data, mode),
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
        (Ok(_), Some(ok), _) => eprintln!("{}", style::str(ok, style::Options::Title)),
        (Err(err), _, Some(err_msg)) => eprintln!(
            "{}: {}",
            style::str(err_msg, style::Options::Muted),
            style::str(&err.to_string(), style::Options::Error)
        ),
        (Err(err), _, None) => eprintln!("{}", style::str(&err.to_string(), style::Options::Error)),
        _ => (),
    }
    result
}
