# Keycard Reader Example

This repository contains source code to a keycard reader example application using libSQL file-based sync.

## Overview

The scenario in this example is the following:

* A keycard reader runs a libSQL embedded replica with limited or air-gapped connectivity.
* A libSQL server runs in a cloud or on-premise server, periodically synchronized using a secure protocol to the keycard reader.

<img src="figures/libsql-sync.png">

## Video

[<img src="https://i.ytimg.com/vi/lQHB_O8WhWo/maxresdefault.jpg" width="50%">](https://www.youtube.com/watch?v=lQHB_O8WhWo "Turso sync demo")

## Running the Demo

### Preparations

First, clone this repository on your machine:

```console
git clone git@github.com:tursodatabase/sync-demo.git
```

Then, in the repository, set up the environment by first installing the libSQL server:

```
brew install libsql/sqld/sqld
```

Then configuring environment variables used in the demo:

```console
source .env
```

and setup some directories:

```console
./setup.sh
```

### Demo

Start the libSQL server:

```bash
./start-server.sh
```

Start the client application:

```bash
./start-client.sh
```

Then write some data to the database:

```bash
turso db shell http://localhost:8080
```

```sql
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

Now upload the incremental snapshots to client, which automatically applies them:

```console
./upload-to-client.sh
```

Update the database again:

```
UPDATE keycards SET expired = 1 WHERE user_id = 1;
```

and upload changes to the client:

```console
./upload-to-client.sh
```

Note, if you want to start the demo from scratch, just delete the data files with:

```console
./teardown.sh && ./setup.sh
```
