import { createClient } from "@libsql/client";
import { prettyPrintDuration } from "./utils.ts"

if (process.env.DB_URL === undefined) {
  throw new Error("DB_URL must be set");
}

if (process.env.AUTH_TOKEN === undefined) {
  throw new Error("AUTH_TOKEN must be set");
}

const client = createClient({
  url: process.env.DB_URL,
  authToken: process.env.AUTH_TOKEN,
  syncUrl: process.env.SYNC_URL,
});

const start = Bun.nanoseconds();
const rs = await client.execute("UPDATE keycards SET EXPIRED = 0 WHERE user_id = 1");
const delta = (Bun.nanoseconds() - start) / 1000;
console.log(`Validated in ${prettyPrintDuration(delta)}`);
