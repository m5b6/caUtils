import axios from "axios";

const BASE_URL =
  "https://reserva.alemana.cl/reserva/portal/resultado/profesional";

const queryParams = new URLSearchParams({
  empresa: "1",
  profesional: "5949123",
  fecha: "16/10/2024",
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

const url = BASE_URL + "?" + queryParams.toString();

axios.get(url).then((response) => {
  console.log(response.data);
});
