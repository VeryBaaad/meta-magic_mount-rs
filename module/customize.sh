#!/system/bin/sh
# Copyright (C) 2026 meta-magic_mount-rs developers
# SPDX-License-Identifier: Apache-2.0

# shellcheck disable=SC2034
SKIPUNZIP=1

if [ -z "$APATCH" ] && [ -z "$KSU" ]; then
  abort "! unsupported root platform"
fi

if [ -n "$KSU_LATE_LOAD" ] && [ -n "$KSU" ]; then
  abort "! unsupported late load mode"
fi

VERSION=$(grep_prop version "${MODPATH}/module.prop")
ui_print "- mmrs version ${VERSION}"

ui_print "- Extracting verify.sh"
unzip -o "$ZIPFILE" 'verify.sh' -d "$TMPDIR" >&2
if [ ! -f "$TMPDIR/verify.sh" ]; then
  ui_print "*********************************************************"
  ui_print "! Unable to extract verify.sh!"
  ui_print "! This zip may be corrupted, please try downloading again"
  abort    "*********************************************************"
fi
# shellcheck disable=SC1091
. "$TMPDIR/verify.sh"

extract 'module.prop'
extract 'config.toml'
extract 'config_apatch.toml'
extract 'metainstall.sh'
extract 'metamount.sh'
extract 'metauninstall.sh'
extract 'uninstal.sh'
extract 'launcher.png'
extract 'meta-mm'
extract 'mazoku'
extract 'machikado'

unzip -o "$ZIPFILE" "webroot/*" -x "*.sha256" -d "$MODPATH"

# Binary is named "meta-mm" in the zip — metamount.sh references it directly.
chmod 755 "$MODPATH/meta-mm" || abort "! Failed to set permissions"

ui_print "- mmrs binary installed"

mkdir -p "/data/adb/magic_mount"

if [ ! -f "/data/adb/magic_mount/config.toml" ]; then
  ui_print "- Add default config"
  if [ -n "${APATCH:-}" ]; then
    cat "$MODPATH/config_apatch.toml" >"/data/adb/magic_mount/config.toml"
  fi

  if [ -n "${KSU:-}" ]; then
    cat "$MODPATH/config.toml" >"/data/adb/magic_mount/config.toml"
  fi

fi

cp "$MODPATH/module.prop" "$MODPATH/module.prop.orig"

ui_print "- Installation complete"
ui_print "- Welcome to mmrs!"
