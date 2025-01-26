#!/bin/env bash

set -e

cd backend
cargo watch \
	-x 'run -p homepage-backend' \
	--watch .. \
	--ignore ../frontend
