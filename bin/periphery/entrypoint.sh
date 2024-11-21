#!/bin/sh
set -e

# Detect the system architecture
ARCH=$(uname -m)

if [ "$ARCH" = "x86_64" ]; then
  exec /usr/local/bin/periphery_amd64 "$@"
elif [ "$ARCH" = "aarch64" ]; then
  exec /usr/local/bin/periphery_arm64 "$@"
else
  echo "Unsupported architecture: $ARCH" >&2
  exit 1
fi