import { createClient } from "@libsql/client";

if (process.env.DB_URL === undefined) {
  throw new Error("DB_URL must be set");
}

if (process.env.AUTH_TOKEN === undefined) {
  throw new Error("AUTH_TOKEN must be set");
}

function prettyPrintDuration(duration) {
  if (duration >= 1000) {
    return (duration / 1000).toFixed(0) + "ms";
  } else {
    return duration.toFixed(0) + "us";
  }
}

const client = createClient({
  url: process.env.DB_URL,
  authToken: process.env.AUTH_TOKEN,
  syncUrl: process.env.SYNC_URL,
});

if (process.env.SYNC_URL != undefined) {
  await client.sync();
}

const queries = 25;
const start = Bun.nanoseconds();
for (let i = 0; i < queries; i++) {
  const rs = await client.execute(
    "SELECT u.name, k.expired FROM users u JOIN keycards as k ON u.user_id = k.user_id",
  );

  for (const row of rs.rows) {
    console.log(
      `The keycard for user ${row.name} is ${
        row.expired ? "expired" : "valid"
      }`,
    );
  }
}
const delta = (Bun.nanoseconds() - start) / 1000;
console.log(
  `took ${prettyPrintDuration(delta)}, ${prettyPrintDuration(
    delta / 50,
  )} per query`,
);
