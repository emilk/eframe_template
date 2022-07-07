#!/usr/bin/env bash
set -eu

PORT=8080

# Starts a local web-server that serves the contents of the `doc/` folder,
# which is the folder to where the web version is compiled.

echo "open http://localhost:$PORT/index.html#dev"

(cd docs && basic-http-server --addr 127.0.0.1:$PORT .)
# (cd docs && python3 -m http.server $PORT --bind 127.0.0.1)
