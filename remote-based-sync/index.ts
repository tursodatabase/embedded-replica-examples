import { createClient } from "@libsql/client";

if (process.env.SYNC_URL === undefined) {
  throw new Error("SYNC_URL must be set");
}

if (process.env.AUTH_TOKEN === undefined) {
  throw new Error("AUTH_TOKEN must be set");
}

const client = createClient({
  url: "file:local.db",
  syncUrl: process.env.SYNC_URL,
  authToken: process.env.AUTH_TOKEN,
});

await client.sync();

const rs = await client.execute("SELECT u.name, k.expired FROM users u JOIN keycards as k ON u.user_id = k.user_id");

for (const row of rs.rows) {
  console.log(`The keycard for user ${row.name} is ${row.expired ? "expired" : "valid"}`);
}
