import connectToDatabase from "./database/database";
import { getAllDoctorIds } from "./database/get_doctor_ids";
import { getDoctorsHours } from "./requests/get_doctors_hours";
import path from "path";
import { Logger } from "./utils/logger";

require("dotenv").config({ path: path.resolve(__dirname, "../.env") });
const logger = Logger.getInstance();
async function main() {
  const db = connectToDatabase();
  const doctorIds = await getAllDoctorIds(db);
  for (const doctorId of doctorIds) {
    const doctorHours = await getDoctorsHours(doctorId);
    console.log(`Doctor ID: ${doctorId}, Hours: ${doctorHours}`);
  }
}

main();
