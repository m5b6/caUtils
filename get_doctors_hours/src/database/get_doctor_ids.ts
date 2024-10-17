import sqlite3, { Database } from "sqlite3";
import { getDoctorsHours } from "../requests/get_doctors_hours";

export async function getAllDoctorIds(db: Database): Promise<number[]> {
  return new Promise((resolve, reject) => {
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
