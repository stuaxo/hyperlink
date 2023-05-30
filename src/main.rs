use std::path::{Path, PathBuf};

use clap::Parser;

use hyperlink::format_path_for_display;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    /// Custom text for the hyperlink.
    #[arg(short, long)]
    pub text: Option<String>,

    /// If not using custom text, file path is relative to specified folder.
    #[arg(short, long)]
    pub relative_to: Option<PathBuf>,

    /// If not using custom text, shorten the file path by using tilde.
    #[arg(short, long)]
    pub shorten: bool,

    /// Always output osc8 hyperlinks even if stdout is not a terminal.
    #[arg(long)]
    pub osc8: bool,

    /// Absolute path of file to link to.
    pub file: PathBuf,
}

pub fn is_output_terminal() -> bool {
    std::env::var("TERM").is_ok()
}

pub fn osc8_hyperlink(filepath: &Path, text: &str) -> String {
    format!(
        "\x1B]8;;file://{}\x07{}\x1B]8;;\x07",
        filepath.display(),
        text
    )
}

fn main() {
    let cli = Cli::parse();
    let filepath = cli.file.as_path();
    // If cli.text is not supplied, generate the file path used in the hyperlink.
    let generated_path = if cli.text.is_none() {
        Some(format_path_for_display(filepath,
                                     cli.relative_to.as_deref(), cli.shorten))
    } else {
        None
    };
    let text = cli.text.unwrap_or_else(|| generated_path.unwrap());

    if cli.osc8 || is_output_terminal() {
        println!("{}", text);
    } else {
        println!("{}", osc8_hyperlink(&filepath, &text));
    }
}
