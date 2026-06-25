#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::doc_markdown,
    clippy::too_many_lines,
    clippy::cast_possible_wrap
)]

mod apk_sign;
mod assets;
mod boot_patch;

#[cfg(target_os = "android")]
mod bunnyhide;

#[cfg(target_os = "android")]
mod cli;
#[cfg(not(target_os = "android"))]
mod cli_non_android;
#[cfg(target_os = "android")]
mod debug;
mod defs;
#[cfg(target_os = "android")]
mod feature;
#[cfg(target_os = "android")]
mod init_event;
#[cfg(target_os = "android")]
mod ksucalls;
#[cfg(target_os = "android")]
mod late_load;
#[cfg(target_os = "android")]
mod magica;
#[cfg(target_os = "android")]
mod metamodule;
#[cfg(target_os = "android")]
mod module;
#[cfg(target_os = "android")]
mod module_config;
#[cfg(target_os = "android")]
mod profile;
#[cfg(target_os = "android")]
mod resetprop;
#[cfg(target_os = "android")]
mod restorecon;
#[cfg(target_os = "android")]
mod sepolicy;
#[cfg(target_os = "android")]
mod su;
#[cfg(target_os = "android")]
mod sulog;
#[cfg(target_os = "android")]
mod unload;
#[cfg(target_os = "android")]
mod utils;

#[cfg(target_os = "android")]
#[allow(nonstandard_style, unused, unsafe_op_in_unsafe_fn)]
mod ksu_uapi;

fn main() -> anyhow::Result<()> {
    #[cfg(target_os = "android")]
    {
        if should_init_bunnyhide() && is_root() {
            bunnyhide::init_random_path()?;
        }

        cli::run()
    }

    #[cfg(not(target_os = "android"))]
    {
        cli_non_android::run()
    }
}

#[cfg(target_os = "android")]
fn should_init_bunnyhide() -> bool {
    let Some(command) = std::env::args().nth(1) else {
        return false;
    };

    matches!(
        command.as_str(),
        "bunnyhide"
            | "hide"
            | "daemon"
            | "service"
            | "post-fs-data"
            | "late-start"
            | "boot-completed"
    )
}

#[cfg(target_os = "android")]
fn is_root() -> bool {
    let Ok(status) = std::fs::read_to_string("/proc/self/status") else {
        return false;
    };

    for line in status.lines() {
        if !line.starts_with("Uid:") {
            continue;
        }

        let mut fields = line.split_whitespace();

        let _ = fields.next();

        let real_uid = fields.next();
        let effective_uid = fields.next().or(real_uid);

        return effective_uid == Some("0");
    }

    false
}