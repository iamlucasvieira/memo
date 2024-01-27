use colored::{ColoredString, Colorize};

pub enum Options {
    Title,
    Error,
    Muted,
}

/// Prints a title in the terminal
pub fn str(text: &str, option: Options) -> ColoredString {
    match option {
        Options::Title => text.green().bold(),
        Options::Error => text.red(),
        Options::Muted => text.dimmed(),
    }
}
