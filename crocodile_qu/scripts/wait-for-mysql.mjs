import net from "node:net";

const host = process.env.MYSQL_HOST ?? "127.0.0.1";
const port = Number(process.env.MYSQL_PORT ?? "3307");
const timeoutMs = Number(process.env.MYSQL_WAIT_TIMEOUT_MS ?? "60000");
const startedAt = Date.now();

function ping() {
  return new Promise((resolve, reject) => {
    const socket = net.createConnection({ host, port }, () => {
      socket.end();
      resolve();
    });

    socket.setTimeout(2000);
    socket.on("error", reject);
    socket.on("timeout", () => {
      socket.destroy();
      reject(new Error("timeout"));
    });
  });
}

while (Date.now() - startedAt < timeoutMs) {
  try {
    await ping();
    console.log(`MySQL is reachable at ${host}:${port}`);
    process.exit(0);
  } catch {
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
}

console.error(`Timed out waiting for MySQL at ${host}:${port}`);
process.exit(1);
