## Getting Started

```console
turso db create sync-example
```

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

```console
export SYNC_URL=$(turso db show --url sync-example)
export AUTH_TOKEN=$(turso db tokens create sync-example)
```

```
UPDATE keycards SET expired = 1 WHERE user_id = 1;
```
