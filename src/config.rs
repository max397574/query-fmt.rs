use std::path::{Path, PathBuf};

pub enum ListForm {
    /// Don't keep opening and closing brackets on separate lines
    Compact,
    /// Keep opening and closing brackets on separate lines
    Extended,
}

pub struct Config {
    /// How to display lists
    list_form: ListForm,
}

static CONFIG_FILE_NAME: [&str; 2] = ["query_fmt.toml", ".query_fmt.toml"];

/// Searches the directory for the configuration toml file
fn find_toml_file(directory: &Path) -> Option<PathBuf> {
    for name in &CONFIG_FILE_NAME {
        let file_path = directory.join(name);
        if file_path.exists() {
            return Some(file_path);
        }
    }
    None
}
