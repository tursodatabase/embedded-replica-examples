#!/bin/bash

if [[ -z "$LIBSQL_SERVER_DIR" ]]; then
   echo "The 'LIBSQL_SERVER_DIR' environment variable is not set. Did you forget to run 'source .env' command?"
   exit 1
fi

if [[ -z "$LIBSQL_CLIENT_SNAPSHOT_DIR" ]]; then
   echo "The 'LIBSQL_CLIENT_SNAPSHOT_DIR' environment variable is not set. Did you forget to run 'source .env' command?"
   exit 1
fi

cp "$LIBSQL_SERVER_DIR"/data.sqld/dbs/default/snapshots/* "$LIBSQL_CLIENT_SNAPSHOT_DIR"

echo "Upload done"
