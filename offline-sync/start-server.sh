#!/bin/bash

if [[ -z "$LIBSQL_SERVER_DIR" ]]; then
   echo "The 'LIBSQL_SERVER_DIR' environment variable is not set. Did you forget to run 'source .env' command?"
   exit 1
fi

SNAPSHOT_PERIOD=5

echo "Starting libSQL server..."

echo "Generating incremental snapshots every $SNAPSHOT_PERIOD seconds"

cd "$LIBSQL_SERVER_DIR" && sqld --max-log-duration "$SNAPSHOT_PERIOD"
