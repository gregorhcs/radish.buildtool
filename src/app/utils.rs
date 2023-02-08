use std::path::PathBuf;

// ----------------------------------------------
pub fn radish_dir_check(projectdir: PathBuf) -> Option<PathBuf> {
    if projectdir.join("full.rebuild.bat").exists()
        { Some(projectdir) } 
    else 
        { None }
}

