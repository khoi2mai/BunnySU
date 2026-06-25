use anyhow::Result;

pub const SULOG_CONFIG_MODULE_ID: &str = "internal.ksud.sulogd";

pub fn run_sulogd() -> Result<()> {
    log::info!("sulogd is disabled at build time");
    Ok(())
}

pub fn spawn_sulogd() -> Result<()> {
    log::info!("sulogd spawn skipped because sulogd is disabled at build time");
    Ok(())
}

pub fn ensure_sulogd_running() -> Result<()> {
    log::info!("sulogd ensure skipped because sulogd is disabled at build time");
    Ok(())
}