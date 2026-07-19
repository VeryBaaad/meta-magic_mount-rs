// Copyright (C) 2026 meta-magic_mount-rs developers
// SPDX-License-Identifier: GPL-v3

use super::*;

#[test]
fn resolves_mirror_paths_for_missing_target() {
    let tempdir = tempfile::tempdir().unwrap();
    let target = tempdir.path().join("missing/parent/target");

    let (ancestor, relative_target) = mirror_paths(&target).unwrap();
    assert_eq!(ancestor, tempdir.path());
    assert_eq!(relative_target, Path::new("missing/parent/target"));
}

#[test]
fn creates_missing_file_in_mirror() {
    let tempdir = tempfile::tempdir().unwrap();
    let source = tempdir.path().join("source");
    fs::File::create(&source).unwrap();
    let target = tempdir.path().join("mirror/parent/target");

    create_mirror_target(&source, &target).unwrap();
    assert!(target.is_file());
}

#[test]
fn creates_missing_directory_in_mirror() {
    let tempdir = tempfile::tempdir().unwrap();
    let source = tempdir.path().join("source");
    fs::create_dir(&source).unwrap();
    let target = tempdir.path().join("mirror/parent/target");

    create_mirror_target(&source, &target).unwrap();
    assert!(target.is_dir());
}
