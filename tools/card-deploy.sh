#!/bin/sh
# Build slot for the H700 and put it on the RG28XX's SD card.
# Usage: tools/card-deploy.sh [--no-build] [--eject]
# CARD defaults to /Volumes/BASEOS.
set -eu

CARD="${CARD:-/Volumes/BASEOS}"
SYS="$CARD/System"
BIN=target-device/device/slot
build=1
eject=0
for arg in "$@"; do
	case "$arg" in
	--no-build) build=0 ;;
	--eject) eject=1 ;;
	*) echo "unknown option: $arg" >&2; exit 2 ;;
	esac
done

cd "$(dirname "$0")/.."
[ -d "$SYS" ] || { echo "no slot card at $CARD (set CARD=...)" >&2; exit 1; }

if [ "$build" = 1 ]; then
	docker run --rm --platform linux/arm64 -v "$PWD":/src -w /src rust:1-bullseye sh -c \
		'cargo build --profile device --target-dir target-device -p slot --no-default-features --features device'
fi
file "$BIN" | grep -q aarch64 || { echo "$BIN is not an aarch64 build" >&2; exit 1; }

[ -f "$SYS/slot" ] && cp -X "$SYS/slot" "$SYS/slot.prev"
cp -X "$BIN" "$SYS/slot"
cp -X card/System/*.sh "$SYS/"
sync

echo "slot $(git describe --tags --always --dirty) on $CARD, previous binary in System/slot.prev"
if [ "$eject" = 1 ]; then
	diskutil eject "$CARD"
fi
