use std::env;
use std::path::PathBuf;

// expands paths with the tilde prefix
pub fn expand_path(path: &str) -> PathBuf {
    if let Some(tilde_expansion_stripped) = path.strip_prefix("~/") {
        if let Some(home) = env::var_os("HOME") {
            return PathBuf::from(home).join(tilde_expansion_stripped);
        }
    }

    PathBuf::from(path)
}
