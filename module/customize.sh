#!/system/bin/sh

ui_print "- Using mmrs metainstall"

if [ -z "$APATCH" ] && [ -z "$KSU" ]; then
  abort "! unsupported root platform"
fi

VERSION=$(grep_prop version "${MODPATH}/module.prop")
ui_print "- mmrs version ${VERSION}"

ui_print "- Detecting device architecture..."

ABI=$(getprop ro.product.cpu.abi)

if [ -z "$ABI" ]; then
  abort "! Failed to detect device architecture"
fi

ui_print "- Device platform: $ABI"

case "$ABI" in
arm64-v8a)
  ui_print "- Selected architecture: arm64-v8a"
  ARCH_BINARY="magic_mount_rs.aarch64"
  ;;
armeabi-v7a)
  ui_print "- Selected architecture: armeabi-v7a"
  ARCH_BINARY="magic_mount_rs.armv7"
  ;;
x86_64)
  ui_print "- Selected architecture: x86_64"
  ARCH_BINARY="magic_mount_rs.x64"
  ;;
*)
  abort "! Unsupported platform: $ABI"
  ;;
esac

ui_print "- Installing architecture-specific binary"

# Rename the selected binary to the generic name
mv "$MODPATH/bin/$ARCH_BINARY" "$MODPATH/meta-mm" || abort "! Failed to rename binary"
rm -rf "$MODPATH/bin"

# Ensure the binary is executable
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

rm -f "$MODPATH/config_apatch.toml"
ui_print "- Remove useless files"

ui_print "- Installation complete"
ui_print "- Image is ready for module installations"

ui_print "- Welcome to mmrs!"
