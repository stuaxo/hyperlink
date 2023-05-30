use std::path::Path;

// Function to generate the file path used in the hyperlink.
pub fn format_path_for_display(
    filepath: &Path,
    relative_to: Option<&Path>,
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
