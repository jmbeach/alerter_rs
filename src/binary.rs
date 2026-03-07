use sha2::{Digest, Sha256};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub const ALERTER_BINARY: &[u8] = include_bytes!(env!("ALERTER_BINARY_PATH"));

pub fn extract_binary() -> Result<PathBuf, std::io::Error> {
    let hash = {
        let mut hasher = Sha256::new();
        hasher.update(ALERTER_BINARY);
        let result = hasher.finalize();
        format!("{:x}", result).chars().take(16).collect::<String>()
    };

    let cache_dir = dirs::cache_dir()
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "No cache directory found")
        })?
        .join("alerter_rs");

    fs::create_dir_all(&cache_dir)?;

    let binary_path = cache_dir.join(format!("alerter-{hash}"));

    if binary_path.exists() {
        return Ok(binary_path);
    }

    fs::write(&binary_path, ALERTER_BINARY)?;
    fs::set_permissions(&binary_path, fs::Permissions::from_mode(0o755))?;

    Ok(binary_path)
}

pub fn get_binary_path() -> Result<PathBuf, std::io::Error> {
    extract_binary()
}
