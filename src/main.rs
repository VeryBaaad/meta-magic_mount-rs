#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]

mod config;
mod defs;
#[cfg(any(target_os = "linux", target_os = "android"))]
mod ksu;
mod magic_mount;
mod scanner;
mod utils;

use std::io::Write;

use anyhow::{Context, Result};
use env_logger::Builder;
use mimalloc::MiMalloc;
use rustix::mount::{MountFlags, mount};

use crate::config::Config;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn init_logger(verbose: bool) {
    let level = if verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    let mut builder = Builder::new();

    builder.format(|buf, record| {
        writeln!(
            buf,
            "[{}] [{}] {}",
            record.level(),
            record.target(),
            record.args()
        )
    });
    builder.filter_level(level).init();

    log::info!("log level: {}", level.as_str());
}

fn main() -> Result<()> {
    let config = Config::load().unwrap_or_default();

    let args: Vec<_> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "scan" => {
                let json_output = args.len() > 2 && args[2] == "--json";

                let modules = scanner::scan_modules(&config.moduledir);

                if json_output {
                    let json = serde_json::to_string(&modules)?;
                    println!("{json}");
                } else {
                    for module in modules {
                        println!("{}", module.id);
                    }
                }
                return Ok(());
            }
            "version" => {
                println!("{{ \"version\": \"{}\" }}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            _ => {}
        }
    }

    init_logger(config.verbose);

    if !ksu::check_ksu() {
        log::error!("only support KernelSU!!");
        panic!();
    }

    log::info!("Magic Mount Starting");
    log::info!("config info:\n{config}");

    log::debug!(
        "current selinux: {}",
        std::fs::read_to_string("/proc/self/attr/current")?
    );

    let tempdir = utils::select_temp_dir().context("failed to select temp dir automatically")?;

    utils::ensure_dir_exists(&tempdir)?;

    if let Err(e) = mount(
        &config.mountsource,
        &tempdir,
        "tmpfs",
        MountFlags::empty(),
        None,
    ) {
        log::error!("mount tmpfs failed: {e}");
    }

    let result = magic_mount::magic_mount(
        &tempdir,
        &config.moduledir,
        &config.mountsource,
        &config.partitions,
        config.umount,
    );

    match result {
        Ok(()) => {
            log::info!("Magic Mount Completed Successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("Magic Mount Failed");
            for cause in e.chain() {
                log::error!("{cause:#?}");
            }
            log::error!("{:#?}", e.backtrace());
            Err(e)
        }
    }
}
