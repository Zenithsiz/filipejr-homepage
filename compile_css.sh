#!/bin/env bash

set -e

# Output directory (for the sass -> css compilation) and file (for merging of all css)
OUTPUT_DIR="target/css"
OUTPUT_FILE="target/css/output.css"

# Compile all css to `target/css`
# TODO: Don't remove the output directory and use `sass --update`
#       once we can bundle only the generated files and ignore any
#       old files no longer in `css/`.
rm -rf "$OUTPUT_DIR"
sass "css/:$OUTPUT_DIR" \
	--no-source-map \
	--load-path="css/" \
	--style=compressed \

# Then merge them
# TODO: Find a proper merging tool?
find "$OUTPUT_DIR" -iname '*.css' -not -path "$OUPUT_FILE" -exec cat {} \+ | tr -d '\n' > "$OUTPUT_FILE"
