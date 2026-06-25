use std::fs;
use std::path::PathBuf;

pub struct BunnyHide {
    runtime_path: PathBuf,
}

impl BunnyHide {
    pub fn init() -> std::io::Result<Self> {
        let path = PathBuf::from(format!("/dev/.bny_{}", random_hex()));
        fs::create_dir_all(&path)?;
        Ok(Self { runtime_path: path })
    }

    pub fn runtime_path(&self) -> &PathBuf {
        &self.runtime_path
    }
}

fn random_hex() -> String {
    use std::fs::File;
    use std::io::Read;

    let mut buf = [0u8; 4];
    if let Ok(mut f) = File::open("/dev/urandom") {
        let _ = f.read_exact(&mut buf);
    }

    format!("{:02x}{:02x}{:02x}{:02x}", buf[0], buf[1], buf[2], buf[3])
}