#!/system/bin/sh

MODDIR="${0%/*}"

BINARY="$MODDIR/meta-mm"

if [ ! -f "$BINARY" ]; then
  log "ERROR: Binary not found: $BINARY"
  exit 1
fi

$BINARY emulated-soft-reboot

EXIT_CODE=$?

exit $EXIT_CODE
