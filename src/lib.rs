use std::path::PathBuf;

// Function to generate the file path used in the hyperlink.
pub fn generate_file_path(
    filepath: &PathBuf,
    relative_to: &Option<PathBuf>,
    shorten: bool,
) -> String {
    let mut text = filepath.to_string_lossy().to_string();

    // If the `relative_to` option is provided, make the filepath relative to the specified path.
    if let Some(relative_to) = relative_to {
        if let Ok(stripped) = filepath.strip_prefix(relative_to) {
            text = stripped.to_string_lossy().to_string();
        }
    }

    if shorten {
        if let Some(home_dir) = dirs::home_dir() {
            text = text.replace(&*home_dir.to_string_lossy(), "~");
        }
    }

    text
}

pub fn get_displayed_text(custom_text: &Option<String>, filepath: &str) -> String {
    // If a custom_text is supplied, return it otherwise return the supplied file path.
    custom_text.clone().unwrap_or_else(|| filepath.to_string())
}
