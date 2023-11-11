use std::env;
use std::path::PathBuf;

/// Retrieves the path to the directory where navigation items are stored.
/// This function assumes that there is an environment variable `NAV_ITEMS_DIR`
/// that contains the path to the navigation items directory.
pub fn get_nav_items_path() -> PathBuf {
    let nav_items_dir = env::var("NAV_ITEMS_DIR")
        .expect("Expected an environment variable named `NAV_ITEMS_DIR`.");
    PathBuf::from(nav_items_dir)
}

/// Retrieves the path to the directory where icons are stored.
/// This function assumes that there is an environment variable `ICONS_DIR`
/// that contains the path to the icons directory.
pub fn get_icons_path() -> PathBuf {
    let icons_dir = env::var("ICONS_DIR")
        .expect("Expected an environment variable named `ICONS_DIR`.");
    PathBuf::from(icons_dir)
}

/// Retrieves the path to the directory where markdown documents are stored.
/// This function assumes that there is an environment variable `MARKDOWN_DIR`
/// that contains the path to the markdown documents directory.
pub fn get_markdown_path() -> PathBuf {
    let markdown_dir = env::var("MARKDOWN_DIR")
        .expect("Expected an environment variable named `MARKDOWN_DIR`.");
    PathBuf::from(markdown_dir)
}

// You can add more utility functions here as needed for the project
