import { JSDOM } from "jsdom";

export default function parseDoctorsHours(html: string) {
  const dom = new JSDOM(html);
  const { document } = dom.window;

  const selector = document.querySelector("resultado");
  if (selector) {
    const rawJsonData = selector.getAttribute(":parametros-back");
    if (!rawJsonData) {
      throw new Error(
        "Attribute 'parametros-back' not found in <resultado> element"
      );
    }
    const parsedJsonData = JSON.parse(rawJsonData);
    const firstHour = parsedJsonData.primera_hora;
    if (!firstHour) {
      throw new Error("First hour not found in JSON data");
    }

    return firstHour;
  }
}
