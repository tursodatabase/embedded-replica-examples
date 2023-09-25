#!/bin/bash

if [[ -z "$LIBSQL_CLIENT_SNAPSHOT_DIR" ]]; then
   echo "The 'LIBSQL_CLIENT_SNAPSHOT_DIR' environment variable is not set. Did you forget to run 'source .env' command?"
   exit 1
fi

cargo run "$LIBSQL_CLIENT_SNAPSHOT_DIR"
