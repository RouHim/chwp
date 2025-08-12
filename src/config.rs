use std::path::Path;

use rand::Rng;

/// Holds the application configuration
pub struct Config {
    pub span: bool,
    pub query: String,
}

/// Parse the CLI arguments
/// # Arguments
/// * `args` - The CLI arguments
/// # Returns
/// The parsed configuration
pub fn parse_cli_args(args: Vec<String>) -> Config {
    let span_string = "span".to_string();
    let span = args.contains(&span_string);

    let mut keywords = args;
    remove_element(&mut keywords, span_string);

    let keyword = if keywords.is_empty() {
        "wallpaper".to_string()
    } else {
        choose_random_keyword(keywords)
    };

    Config {
        span,
        query: keyword,
    }
}

/// Choose a random keyword from a list of keywords
/// # Arguments
/// * `keywords` - The list of keywords
/// # Returns
/// A random keyword
fn choose_random_keyword(keywords: Vec<String>) -> String {
    if keywords.len() > 1 {
        let random_index = rand::thread_rng().gen_range(0..keywords.len());
        keywords.get(random_index).unwrap().to_string()
    } else {
        keywords.first().unwrap().to_string()
    }
}

/// Remove an element from a vector
/// # Arguments
/// * `keywords` - The list of keywords
/// * `term` - The element to remove
/// # Returns
/// The list of keywords without the removed element
fn remove_element(keywords: &mut Vec<String>, term: String) {
    let index = keywords.iter().position(|item| *item == term);

    if let Some(index) = index {
        keywords.remove(index);
    }
}

/// Check if a string is a URL
/// # Arguments
/// * `to_check` - The string to check
/// # Returns
/// True if the string is a URL
pub fn is_url(to_check: &str) -> bool {
    to_check.starts_with("http") && to_check.contains("://")
}

/// Expand a leading '~/' to the user's home directory
fn expand_tilde(path: &str) -> String {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest).to_string_lossy().into_owned();
        }
    }
    path.to_string()
}

/// Check if a string is a local path
/// # Arguments
/// * `to_check` - The string to check
/// # Returns
/// True if the string is a local path
pub fn is_local_path(to_check: &str) -> bool {
    let expanded = expand_tilde(to_check);
    Path::new(&expanded).exists()
}
