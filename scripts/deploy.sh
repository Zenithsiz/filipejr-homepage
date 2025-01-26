#!/bin/env bash

set -e

scripts/deploy-frontend.sh &
DEPLOY_FRONTEND=$!
scripts/deploy-backend.sh &
DEPLOY_BACKEND=$!

wait $DEPLOY_FRONTEND
wait $DEPLOY_BACKEND
