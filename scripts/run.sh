#!/bin/env bash

set -e

parallel --ungroup ::: \
	scripts/run-frontend.sh \
	scripts/run-backend.sh
