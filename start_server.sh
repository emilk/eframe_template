#!/bin/bash
set -eu

(cd docs && python3 -m http.server 8080 --bind 127.0.0.1)
