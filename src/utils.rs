use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow, bail};
#[cfg(any(target_os = "linux", target_os = "android"))]
use extattr::{Flags as XattrFlags, lsetxattr};
use regex_lite::Regex;

#[cfg(any(target_os = "linux", target_os = "android"))]
use crate::defs::SELINUX_XATTR;
use crate::defs::TMPFS_CANDIDATES;

/// Validate module_id format and security
/// Module ID must match: ^[a-zA-Z][a-zA-Z0-9._-]+$
/// - Must start with a letter (a-zA-Z)
/// - Followed by one or more alphanumeric, dot, underscore, or hyphen characters
/// - Minimum length: 2 characters
pub fn validate_module_id(module_id: &str) -> Result<()> {
    let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9._-]+$")?;
    if re.is_match(module_id) {
        Ok(())
    } else {
        Err(anyhow!(
            "Invalid module ID: '{}'. Must match /^[a-zA-Z][a-zA-Z0-9._-]+$/",
            module_id
        ))
    }
}

pub fn lsetfilecon<P: AsRef<Path>>(path: P, con: &str) -> Result<()> {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    lsetxattr(&path, SELINUX_XATTR, con, XattrFlags::empty()).with_context(|| {
        format!(
            "Failed to change SELinux context for {}",
            path.as_ref().display()
        )
    })?;
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn lgetfilecon<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let con = extattr::lgetxattr(&path, SELINUX_XATTR).with_context(|| {
        format!(
            "Failed to get SELinux context for {}",
            path.as_ref().display()
        )
    })?;
    let con = String::from_utf8_lossy(&con);
    Ok(con.to_string())
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
pub fn lgetfilecon<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    unimplemented!()
}

pub fn ensure_dir_exists<P>(dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let result = create_dir_all(&dir);
    if dir.as_ref().is_dir() && result.is_ok() {
        Ok(())
    } else {
        bail!("{} is not a regular directory", dir.as_ref().display())
    }
}

fn is_ok_empty<P>(dir: P) -> bool
where
    P: AsRef<Path>,
{
    dir.as_ref()
        .read_dir()
        .is_ok_and(|mut entries| entries.next().is_none())
}

pub fn select_temp_dir() -> Result<PathBuf> {
    for candidate in TMPFS_CANDIDATES {
        let path = Path::new(candidate);

        if !path.exists() {
            continue;
        }

        if is_ok_empty(path) {
            log::info!("selected tmpfs: {}", path.display(),);
            return Ok(path.to_path_buf());
        }
    }

    bail!(
        "no tmpfs found in candidates: {}",
        TMPFS_CANDIDATES.join(", ")
    )
}
