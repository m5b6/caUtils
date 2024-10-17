import sqlite3, { Database } from "sqlite3";

export default function connectToDatabase() {
  const dbUrl = process.env.DB_URL || "";
  const db = new sqlite3.Database(dbUrl, sqlite3.OPEN_READONLY, (err: any) => {
    if (err) {
      console.error("Could not connect to database", err);
    }
  });
  return db;
}
