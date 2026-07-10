use std::env;
use std::path::PathBuf;

/// Resolve Super instance root: `SUPER_ROOT` → exe-relative layout → cwd.
///
/// Shared by superd and licensed plugins so config paths stay consistent.
pub fn resolve_super_root() -> PathBuf {
    if let Ok(p) = env::var("SUPER_ROOT") {
        return PathBuf::from(p);
    }

    if let Ok(exe_path) = env::current_exe() {
        if let Some(bin_dir) = exe_path.parent() {
            if let Some(root) = bin_dir.parent() {
                if root.join("bin").exists() {
                    return root.to_path_buf();
                }
            }
        }
    }

    PathBuf::from(".")
}
