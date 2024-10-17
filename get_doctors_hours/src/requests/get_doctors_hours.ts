import axios from "axios";
import moment from "moment";
import parseDoctorsHours from "./parser";
import { Logger } from "../utils/logger";

export const BASE_URL =
  "https://reserva.alemana.cl/reserva/portal/resultado/profesional";

const logger = Logger.getInstance();

export async function getDoctorsHours(doctorId: number) {
  const url = BASE_URL + "?" + buildQueryParams(doctorId).toString();
  logger.log(`Requesting doctor ID ${doctorId}: ${url}`);
  try {
    const response = await axios.get(url);
    logger.success(`Successfully fetched doctor ID ${doctorId}!`);
    const parsedResponse = parseDoctorsHours(response.data);
    return parsedResponse;
  } catch (error) {
    logger.error(`Error fetching doctor ID ${doctorId}`);
  }
}

export function buildQueryParams(doctorId: number) {
  return new URLSearchParams({
    empresa: "1",
    profesional: doctorId.toString(),
    fecha: moment().format("DD/MM/YYYY"),
    tipo_busqueda: "1",
    area_interes: "0",
    ubicacion: "99",
    tipo_di: "01",
    di: Math.floor(Math.random() * 1000000).toString(),
    super_centro: "0",
    centro: "0",
    horario: "A",
    idioma: "0",
  });
}
