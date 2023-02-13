// ----------------------------------------------
use std::path::PathBuf;

use super::config;

// ----------------------------------------------
pub fn pretty_print<P>(path: P) -> String 
    where P: Into<PathBuf> 
{
    path
        .into()
        .iter()
        .skip(0)
        .map(|os_str| String::from(
            os_str.
                to_str().
                unwrap().
                trim_matches(|c| c == '\\' || c == '?'))
        )
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .join("\\")
}

// ----------------------------------------------
pub fn radish_dir_check(projectdir: PathBuf) -> Option<PathBuf> {
    if projectdir.join("full.rebuild.bat").exists() {
        match projectdir.canonicalize() {
            Ok(dir) => {
                config::set_most_recent_projectpath(&dir);
                Some(dir)
            },
            Err(_) => None
        }
    }
    else 
        { None }
}
