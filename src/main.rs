use clap::Parser;
use std::path::{Path, PathBuf};

use hyperlink::{generate_file_path, get_displayed_text};

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    /// Set custom text for the hyperlink
    #[arg(short, long)]
    pub text: Option<String>,

    /// Make the file path relative to the specified folder
    #[arg(short, long)]
    pub relative_to: Option<PathBuf>,

    /// Shorten the file path by using tilde instead of the home directory
    #[arg(short, long)]
    pub shorten: bool,

    /// The file or directory to generate a hyperlink for
    pub file: PathBuf,
}

pub fn is_output_terminal() -> bool {
    std::env::var("TERM").is_ok()
}

pub fn print_hyperlink(filepath: &Path, text: &str) {
    if is_output_terminal() {
        println!(
            "\x1B]8;;file://{}\x07{}\x1B]8;;\x07",
            filepath.display(),
            text
        );
    } else {
        println!("{}", text);
    }
}

fn main() {
    let cli = Cli::parse();
    let filepath = cli.file.as_path();
    // Generate the file path used in the hyperlink.
    let generated_path = generate_file_path(filepath,
                                        cli.relative_to.as_deref(), cli.shorten);

    // Generate the text to be displayed for the hyperlink.
    let text = get_displayed_text(&cli.text, &generated_path);

    // Print the hyperlink to the console.
    print_hyperlink(&filepath, &text);
}
