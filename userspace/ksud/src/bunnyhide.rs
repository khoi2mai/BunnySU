use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::sync::OnceLock;

static RUNTIME_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn init_random_path() -> anyhow::Result<()> {
    let path = PathBuf::from(format!("/dev/.bny_{}", random_hex()));

    fs::create_dir_all(&path)?;

    let _ = RUNTIME_PATH.set(path);

    Ok(())
}

pub fn runtime_path() -> Option<&'static PathBuf> {
    RUNTIME_PATH.get()
}

fn random_hex() -> String {
    let mut buf = [0u8; 4];

    if let Ok(mut f) = fs::File::open("/dev/urandom") {
        let _ = f.read_exact(&mut buf);
    }

    format!(
        "{:02x}{:02x}{:02x}{:02x}",
        buf[0], buf[1], buf[2], buf[3]
    )
}