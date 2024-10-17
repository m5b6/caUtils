use html_escape::decode_html_entities;
use scraper::{Html, Selector};
use serde_json::Value;
use std::error::Error;

pub fn parse_resultado_parametros(html_content: &str) -> Result<Value, Box<dyn Error>> {
    let document = Html::parse_document(html_content);

    let selector = Selector::parse("resultado").unwrap();
    if let Some(element) = document.select(&selector).next() {
        if let Some(parametros_back) = element.value().attr(":parametros-back") {
            let decoded = decode_html_entities(parametros_back);

            let json_data: Value = serde_json::from_str(&decoded)?;

            return Ok(json_data);
        } else {
            return Err("Attribute ':parametros-back' not found in <resultado> element".into());
        }
    } else {
        return Err("<resultado> element not found in HTML".into());
    }
}
