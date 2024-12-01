#!/bin/env bash

set -e

# Remote
REMOTE="homepage@filipejr.com"
REMOTE_PATH="/home/homepage/www/"

# Build
trunk build --release

# Copy to remote
rsync -rzv "dist/" "$REMOTE:$REMOTE_PATH"
ssh "$REMOTE" "chown -R homepage:www-data '$REMOTE_PATH' && chmod -R 1750 '$REMOTE_PATH'"
