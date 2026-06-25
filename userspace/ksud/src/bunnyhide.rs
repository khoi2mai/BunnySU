use anyhow::{Context, Result, anyhow};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static RUNTIME_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn init_random_path() -> Result<()> {
    let path = PathBuf::from(format!("/dev/.bny_{}", random_hex()?));

    fs::create_dir_all(&path)
        .with_context(|| format!("failed to create bunnyhide runtime path: {}", path.display()))?;

    let _ = RUNTIME_PATH.set(path);

    Ok(())
}

pub fn runtime_path() -> Option<&'static PathBuf> {
    RUNTIME_PATH.get()
}

pub fn require_runtime_path() -> Result<&'static PathBuf> {
    runtime_path().ok_or_else(|| anyhow!("bunnyhide runtime path is not initialized"))
}

pub fn runtime_file(name: &str) -> Result<PathBuf> {
    if name.contains('/') || name.contains("..") {
        return Err(anyhow!("invalid runtime file name: {name}"));
    }

    Ok(require_runtime_path()?.join(name))
}


pub fn runtime_dir(name: &str) -> Result<PathBuf> {
    if name.contains('/') || name.contains("..") {
        return Err(anyhow!("invalid runtime dir name: {name}"));
    }

    let path = require_runtime_path()?.join(name);

    fs::create_dir_all(&path)
        .with_context(|| format!("failed to create runtime dir: {}", path.display()))?;

    Ok(path)
}

pub fn cleanup_random_path() -> Result<()> {
    if let Some(path) = runtime_path() {
        if Path::new(path).exists() {
            fs::remove_dir_all(path)
                .with_context(|| format!("failed to cleanup runtime path: {}", path.display()))?;
        }
    }

    Ok(())
}

fn random_hex() -> Result<String> {
    let mut buf = [0u8; 4];

    let mut f = fs::File::open("/dev/urandom").context("failed to open /dev/urandom")?;

    f.read_exact(&mut buf)
        .context("failed to read random bytes from /dev/urandom")?;

    Ok(format!(
        "{:02x}{:02x}{:02x}{:02x}",
        buf[0], buf[1], buf[2], buf[3]
    ))
}