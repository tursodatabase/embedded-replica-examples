#!/bin/bash

if [[ -z "$LIBSQL_SERVER_DIR" ]]; then
   echo "The 'LIBSQL_SERVER_DIR' environment variable is not set. Did you forget to run 'source .env' command?"
   exit 1
fi

if [[ -z "$LIBSQL_CLIENT_SNAPSHOT_DIR" ]]; then
   echo "The 'LIBSQL_CLIENT_SNAPSHOT_DIR' environment variable is not set. Did you forget to run 'source .env' command?"
   exit 1
fi

rm test.db
rm -rf "$LIBSQL_SERVER_DIR"
rm -rf "$LIBSQL_CLIENT_SNAPSHOT_DIR"

echo "Teardown done"
