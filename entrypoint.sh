#!/bin/sh
set -e

nginx

exec /app/m0n1t0r-server /app/config.toml
