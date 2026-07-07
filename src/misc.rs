// Copyright (C) 2026 meta-magic_mount-rs developers
// SPDX-License-Identifier: GPL-v3

use std::path::Path;

use rustix::mount::{UnmountFlags, unmount};

use machikado_rs::{FileMapping, load_folder_files, verify};

use crate::{defs, utils::ksucalls};

fn init_logger() {
    #[cfg(not(target_os = "android"))]
    {
        use std::io::Write;

        let mut builder = env_logger::Builder::new();

        builder.format(|buf, record| {
            writeln!(
                buf,
                "[{}] [{}] {}",
                record.level(),
                record.target(),
                record.args()
            )
        });
        builder.filter_level(log::LevelFilter::Debug).init();
    }

    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Debug)
                .with_tag("MagicMount"),
        );
    }
}

fn verify_module_safety() -> std::result::Result<(), Box<dyn std::error::Error>> {
    const ORG_PUBLIC: [u8; 32] = [
        0x16, 0xbd, 0x7c, 0x31, 0x1b, 0xad, 0xf0, 0x52, 0xaa, 0x0c, 0x3a, 0xe1, 0x88, 0xf3, 0xf1,
        0x41, 0xb5, 0x04, 0xcb, 0xbf, 0x08, 0x30, 0x61, 0xad, 0x3f, 0xfb, 0x21, 0x5d, 0x61, 0xc7,
        0x40, 0xfe,
    ];
    let machikado = std::fs::read(defs::MACHIKADO_FILE)?;
    let mazoku = std::fs::read(defs::MAZOKU_FILE)?;
    let mapping = FileMapping::from(("module.prop", "module.prop.orig"));
    let entries = load_folder_files(
        Path::new(defs::SELF_MODULE_PATH),
        &[],
        &["machikado", "update", "disable", "remove"],
        Some(&mapping),
    )?;

    match verify(
        &machikado,
        &mazoku,
        &entries,
        env!("MODULE_ID"),
        &ORG_PUBLIC,
    ) {
        (true, _) => {}
        (false, Some(e)) => return Err(Box::new(e)),
        (false, None) => unreachable!(),
    }

    log::info!(
        "module signature verified successfully ({} files)",
        entries.len()
    );
    Ok(())
}

fn init_list() {
    super::parser::COMMAND_LIST
        .get_or_init(|| super::parser::parser_custom(defs::CUSTOM_LIST_PATH));
}

pub fn cleanup<P>(tempdir: P)
where
    P: AsRef<Path>,
{
    if let Err(e) = unmount(
        tempdir.as_ref().to_string_lossy().to_string(),
        UnmountFlags::DETACH,
    ) {
        log::warn!("failed to unmount tempdir: {e}");
    }
    if let Err(e) = std::fs::remove_dir(&tempdir) {
        log::warn!("failed to remove tempdir: {e}");
    }
}

pub fn pre_init() {
    assert!(
        !(std::env::var("KSU_LATE_LOAD").is_ok() && std::env::var("KSU").is_ok()),
        "! unsupported late load mode"
    );

    init_logger();
    if let Err(e) = verify_module_safety() {
        log::error!("module safety verification failed: {e}");
        panic!("module safety verification failed: {e}");
    }
    ksucalls::check_ksu();
    init_list();
}
