# Remote Sync Example with Turso

This repository contains source code to a example application using libSQL remote sync.

## Overview

The scenario in this example is the following:

* A libSQL server runs in a cloud on Turso.
* An application embeds libSQL database, which periodically syncs with the server.

## Getting Started

First, create a database on Turso:

```console
turso db create sync-example
```

Then create the schema and insert some data for this example application:

```console
turso db shell sync-example
```

```
CREATE TABLE users (
    user_id INT PRIMARY KEY,
    name TEXT
);

CREATE TABLE keycards (
    keycard_id INT PRIMARY KEY,
    user_id INT,
    expired BOOL,
    FOREIGN KEY (user_iD) REFERENCES users(user_id)
);

INSERT INTO users VALUES (1, 'Glauber');
INSERT INTO keycards VALUES (1, 1, false);
```

### First step: Running remotely

To run remotely, configure the `DB_URL` and `AUTH_TOKEN` environment variables, while making sure that `SYNC_URL` is not set:

```console
unset SYNC_URL
export DB_URL=$(turso db show --url sync-example)
export AUTH_TOKEN=$(turso db tokens create sync-example)
```

and run the example application:

```
bun index.ts
```

and you should see the following output, repeated 10 times:

```
The keycard for user Glauber is valid. Query took [time]
```


### Second step: Running an embedded replica

Configure the `SYNC_URL` and `AUTH_TOKEN` environment variables, and set `DB_URL` to a local file:

```console
export DB_URL="file:local.db"
export SYNC_URL=$(turso db show --url sync-example)
export AUTH_TOKEN=$(turso db tokens create sync-example)
```

and then run the example application:

```
bun index.ts
```

and you should see the following output, repeated 10 times:

```
The keycard for user Glauber is valid. Query took [time]
```

### Updates

You can now update data on the remote database:

```
UPDATE keycards SET expired = 1 WHERE user_id = 1;
```

and now running the application, either remotely or as an embedded replica, outputs:

```
The keycard for user Glauber is expired. Query took [time]
```
