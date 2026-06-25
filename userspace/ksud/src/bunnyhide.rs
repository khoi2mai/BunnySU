use anyhow::{Context, Result, anyhow, ensure};
use std::env;
use std::fs;
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Duration;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

const DEV_DIR: &str = "/dev";
const RUNTIME_PREFIX: &str = ".bny_";
const RANDOM_HEX_BYTES: usize = 4;
const CLEANUP_MIN_AGE: Duration = Duration::from_secs(60);
pub const RUNTIME_PATH_ENV: &str = "BUNNYSU_RUNTIME_PATH";

static RUNTIME_PATH: OnceLock<PathBuf> = OnceLock::new();
pub fn init_random_path() -> Result<()> {
    if RUNTIME_PATH.get().is_some() {
        return Ok(());
    }

    let path = runtime_path_from_env()?.unwrap_or_else(random_runtime_path);

    fs::create_dir_all(&path)
        .with_context(|| format!("failed to create bunnyhide runtime path: {}", path.display()))?;

    set_private_dir_permissions(&path)?;

    if RUNTIME_PATH.set(path.clone()).is_err() {
        let _ = fs::remove_dir(&path);
        return Ok(());
    }

    env::set_var(RUNTIME_PATH_ENV, path.as_os_str());
    if let Err(err) = cleanup_stale_empty_runtime_dirs(&path) {
        log::debug!("bunnyhide cleanup skipped: {err:#}");
    }

    Ok(())
}

pub fn runtime_path() -> Option<&'static PathBuf> {
    RUNTIME_PATH.get()
}

pub fn require_runtime_path() -> Result<&'static PathBuf> {
    runtime_path().ok_or_else(|| anyhow!("bunnyhide runtime path is not initialized"))
}

pub fn runtime_file(name: &str) -> Result<PathBuf> {
    validate_runtime_name(name)?;
    Ok(require_runtime_path()?.join(name))
}

pub fn runtime_dir(name: &str) -> Result<PathBuf> {
    validate_runtime_name(name)?;

    let path = require_runtime_path()?.join(name);

    fs::create_dir_all(&path)
        .with_context(|| format!("failed to create runtime dir: {}", path.display()))?;

    set_private_dir_permissions(&path)?;

    Ok(path)
}

pub fn cleanup_random_path() -> Result<()> {
    if let Some(path) = runtime_path()
        && path.exists()
    {
        fs::remove_dir_all(path)
            .with_context(|| format!("failed to cleanup runtime path: {}", path.display()))?;
    }

    Ok(())
}

fn runtime_path_from_env() -> Result<Option<PathBuf>> {
    let Ok(value) = env::var(RUNTIME_PATH_ENV) else {
        return Ok(None);
    };

    let value = value.trim();
    if value.is_empty() {
        return Ok(None);
    }

    let path = PathBuf::from(value);

    validate_runtime_path(&path)
        .with_context(|| format!("invalid {RUNTIME_PATH_ENV}: {}", path.display()))?;

    Ok(Some(path))
}

fn random_runtime_path() -> PathBuf {
    Path::new(DEV_DIR).join(format!("{}{}", RUNTIME_PREFIX, random_hex()))
}

fn random_hex() -> String {
    let mut buf = [0u8; RANDOM_HEX_BYTES];

    if let Ok(mut f) = fs::File::open("/dev/urandom") {
        let _ = f.read_exact(&mut buf);
    }

    format!(
        "{:02x}{:02x}{:02x}{:02x}",
        buf[0], buf[1], buf[2], buf[3]
    )
}

fn validate_runtime_name(name: &str) -> Result<()> {
    ensure!(!name.is_empty(), "runtime name is empty");
    ensure!(name != "." && name != "..", "invalid runtime name: {name}");

    let path = Path::new(name);
    ensure!(
        path.file_name().and_then(|file_name| file_name.to_str()) == Some(name),
        "invalid runtime name: {name}"
    );

    Ok(())
}

fn validate_runtime_path(path: &Path) -> Result<()> {
    ensure!(path.is_absolute(), "runtime path must be absolute");

    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("runtime path has no parent"))?;

    ensure!(
        parent == Path::new(DEV_DIR),
        "runtime path must be directly under {DEV_DIR}"
    );

    ensure!(
        is_bunny_runtime_dir(path),
        "runtime path must match /dev/.bny_xxxxxxxx"
    );

    Ok(())
}

fn set_private_dir_permissions(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))
            .with_context(|| format!("failed to chmod {} to 0700", path.display()))?;
    }

    Ok(())
}

fn cleanup_stale_empty_runtime_dirs(keep: &Path) -> Result<()> {
    let dev_dir = Path::new(DEV_DIR);

    for entry in fs::read_dir(dev_dir).with_context(|| format!("failed to read {DEV_DIR}"))? {
        let Ok(entry) = entry else {
            continue;
        };

        let path = entry.path();

        if path == keep {
            continue;
        }

        if !is_bunny_runtime_dir(&path) {
            continue;
        }

        if !path.is_dir() {
            continue;
        }

        if !is_old_enough(&path, CLEANUP_MIN_AGE) {
            continue;
        }

        match fs::remove_dir(&path) {
            Ok(()) => {
                log::debug!("removed stale bunnyhide runtime dir: {}", path.display());
            }
            Err(err)
                if matches!(
                    err.kind(),
                    ErrorKind::NotFound
                        | ErrorKind::DirectoryNotEmpty
                        | ErrorKind::PermissionDenied
                ) => {}
            Err(err) => {
                log::debug!("failed to remove stale runtime dir {}: {err}", path.display());
            }
        }
    }

    Ok(())
}

fn is_bunny_runtime_dir(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    let Some(hex) = name.strip_prefix(RUNTIME_PREFIX) else {
        return false;
    };

    hex.len() == RANDOM_HEX_BYTES * 2 && hex.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn is_old_enough(path: &Path, min_age: Duration) -> bool {
    let Ok(metadata) = fs::metadata(path) else {
        return false;
    };

    let Ok(modified) = metadata.modified() else {
        return false;
    };

    let Ok(elapsed) = modified.elapsed() else {
        return false;
    };

    elapsed >= min_age
}