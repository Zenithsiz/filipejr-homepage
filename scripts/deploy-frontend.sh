#!/bin/env bash

set -e

# Remote
REMOTE="homepage@filipejr.com"
REMOTE_PATH="/home/homepage/www/"

# Build
trunk build \
	--release \
	--config homepage-frontend/Trunk.toml

# Copy to remote
rsync \
	--recursive \
	--compress \
	--verbose \
	--delete \
	"homepage-frontend/dist/" \
	"$REMOTE:$REMOTE_PATH"
ssh "$REMOTE" "chown -R homepage:www-data '$REMOTE_PATH' && chmod -R 1750 '$REMOTE_PATH'"
