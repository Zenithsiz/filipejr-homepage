#!/bin/env bash

set -e

# Remote
REMOTE_FRONTEND="homepage@filipejr.com"
REMOTE_BACKEND="root@filipejr.com"
REMOTE_PATH_FRONTEND="/home/homepage/www/"
REMOTE_PATH_BACKEND="/root/packages/"

# Build
trunk build \
	--release \
	--config frontend/Trunk.toml &
BUILD_FRONTEND=$!
cargo build \
	--release \
	--target x86_64-unknown-linux-musl \
	--package homepage-backend &
BUILD_BACKEND=$!

wait $BUILD_FRONTEND
wait $BUILD_BACKEND

# Copy to remote
rsync \
	--recursive \
	--compress \
	--verbose \
	--delete \
	"frontend/dist/" \
	"$REMOTE_FRONTEND:$REMOTE_PATH_FRONTEND"
ssh "$REMOTE_FRONTEND" "chown -R homepage:www-data '$REMOTE_PATH_FRONTEND' && chmod -R 1750 '$REMOTE_PATH_FRONTEND'"

BACKEND_DEB_PATH=$(
	cargo deb \
		--package homepage-backend \
		--target x86_64-unknown-linux-musl \
		--no-build
)
BACKEND_DEB=$(basename "$BACKEND_DEB_PATH")
scp \
	"$BACKEND_DEB_PATH" \
	"$REMOTE_BACKEND:$REMOTE_PATH_BACKEND"
ssh "$REMOTE_BACKEND" "dpkg -i 'packages/$BACKEND_DEB' && systemctl daemon-reload && systemctl restart homepage-backend"
