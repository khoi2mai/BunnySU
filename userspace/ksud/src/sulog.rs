pub fn run_sulogd() -> Result<()> {
    if SULOGD_DISABLED {
        log::info!("sulogd is disabled at build time");
        return Ok(());
    }

    let Some(_lock_guard) = SulogdLockGuard::acquire()? else {
        log::info!("sulogd lock is held, skipping start");
        return Ok(());
    };

    let mut restart_count = 0u64;
    loop {
        match run_sulog_session(restart_count) {
            Ok(SessionExitReason::FdClosed) => {
                log::warn!(
                    "restarting sulogd session after fd close in {}s",
                    SULOGD_RESTART_DELAY.as_secs()
                );
            }
            Ok(SessionExitReason::EpollHangup) => {
                log::warn!(
                    "restarting sulogd session after hangup in {}s",
                    SULOGD_RESTART_DELAY.as_secs()
                );
            }
            Err(err) => {
                log::warn!(
                    "sulogd session failed: {err:#}; restarting in {}s",
                    SULOGD_RESTART_DELAY.as_secs()
                );
            }
        }

        restart_count = restart_count.saturating_add(1);
        thread::sleep(SULOGD_RESTART_DELAY);
    }
}

pub fn spawn_sulogd() -> Result<()> {
    if SULOGD_DISABLED {
        log::info!("sulogd spawn skipped because sulogd is disabled at build time");
        return Ok(());
    }

    if utils::create_daemon(true)? {
        let current_exe = std::env::current_exe().context("failed to resolve current ksud path")?;
        let lock_path = random_sulogd_lock_path();

        if let Some(parent) = lock_path.parent() {
            ensure_private_dir_exists(parent)?;
        }

        let mut command = Command::new(current_exe);
        command
            .arg("sulogd")
            .env(SULOGD_LOCK_ENV, lock_path.as_os_str())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .current_dir("/");

        if let Some(runtime_path) = bunnyhide::runtime_path() {
            command.env(bunnyhide::RUNTIME_PATH_ENV, runtime_path.as_os_str());
        }

        Err(command.exec()).context("failed to exec sulogd")
    } else {
        Ok(())
    }
}

pub fn ensure_sulogd_running() -> Result<()> {
    if SULOGD_DISABLED {
        log::info!("sulogd ensure skipped because sulogd is disabled at build time");
        return Ok(());
    }

    spawn_sulogd()
}