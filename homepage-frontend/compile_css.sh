#!/bin/env bash

set -e

if [ "$TRUNK_PROFILE" == "debug" ]; then
	# TODO: Once we can make `trunk serve` actually allow
	#       access to the files, don't embed the sources.
	SASS_ARGS="--embed-source-map --embed-sources"
elif [ "$TRUNK_PROFILE" == "release" ]; then
	SASS_ARGS="--no-source-map"
else
	printf "Unknown \$TRUNK_PROFILE: \"$TRUNK_PROFILE\"\n"
	exit 1
fi

sass "css/main.scss" "../target/css/output.css" \
	--load-path="css/" \
	--style=compressed \
	$SASS_ARGS
