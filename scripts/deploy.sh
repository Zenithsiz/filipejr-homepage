#!/bin/env bash

set -e

parallel --ungroup ::: \
	scripts/deploy-frontend.sh \
	scripts/deploy-backend.sh
