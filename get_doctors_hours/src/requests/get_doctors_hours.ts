import axios from "axios";
import { buildQueryParams, BASE_URL } from "./consts";
import parseDoctorsHours from "./parser";

export async function getDoctorsHours(doctorId: number) {
  const url = BASE_URL + "?" + buildQueryParams(doctorId).toString();
  const response = await axios.get(url);
  const parsedResponse = parseDoctorsHours(response.data);
  return parsedResponse;
}
