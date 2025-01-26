#!/bin/env bash

set -e

# Remote
REMOTE="root@filipejr.com"
REMOTE_PATH="/root/packages/"

# Build
cargo build \
	--release \
	--target x86_64-unknown-linux-musl \
	--package homepage-backend

# Copy to remote
DEB_PATH=$(
	cargo deb \
		--package homepage-backend \
		--target x86_64-unknown-linux-musl \
		--no-build
)
DEB=$(basename "$DEB_PATH")
scp \
	"$DEB_PATH" \
	"$REMOTE:$REMOTE_PATH"
ssh "$REMOTE" "dpkg -i 'packages/$DEB' && systemctl daemon-reload && systemctl restart homepage-backend"
