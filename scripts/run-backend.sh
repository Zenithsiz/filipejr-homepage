#!/bin/env bash

set -e

cd homepage-backend/
cargo watch \
	-x 'run -p homepage-backend' \
	--watch .. \
	--ignore ../frontend
