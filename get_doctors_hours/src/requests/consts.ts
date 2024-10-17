import moment from "moment";

export const BASE_URL =
  "https://reserva.alemana.cl/reserva/portal/resultado/profesional";

export function buildQueryParams(doctorId: number) {
  return new URLSearchParams({
    empresa: "1",
    profesional: doctorId.toString(),
    fecha: moment().format("DD/MM/YYYY"),
    tipo_busqueda: "1",
    area_medica: "104",
    area_interes: "0",
    ubicacion: "99",
    tipo_di: "01",
    di: "123123123",
    super_centro: "0",
    centro: "0",
    horario: "A",
    idioma: "0",
  });
}
