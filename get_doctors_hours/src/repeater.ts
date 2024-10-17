import sqlite3 from "sqlite3";
import { getDoctorsHours } from "./requests/get_doctors_hours";

export async function getAllDoctorIds(): Promise<number[]> {
  return new Promise((resolve, reject) => {
    const db = new sqlite3.Database(
      "../get_doctors/doctors.db",
      sqlite3.OPEN_READONLY,
      (err) => {
        if (err) {
          console.error("Could not connect to database", err);
          reject(err);
        }
      }
    );

    db.all(
      "SELECT ppn FROM doctors WHERE ppn IS NOT NULL;",
      [],
      (err, rows) => {
        if (err) {
          console.error("Error querying database", err);
          reject(err);
        } else {
          const doctorIds = rows.map((row: any) => row.ppn);
          resolve(doctorIds);
        }
        db.close();
      }
    );
  });
}

async function main() {
  try {
    const doctorIds = await getAllDoctorIds();
    for (const doctorId of doctorIds) {
      const doctorHours = await getDoctorsHours(doctorId);
      console.log(`Doctor ID: ${doctorId}, Hours: ${doctorHours}`);
    }
  } catch (error) {
    console.error("Error fetching doctor IDs or hours:", error);
  }
}

main();
