use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Doctor {
    pub id: u64,
    pub nombres: String,
    pub apellidoMaterno: String,
    pub apellidoPaterno: String,
    pub rut: u64,
    #[serde(default)]
    pub codigoAreaMedica: String,
    pub descripcionAreaMedica: String,
    #[serde(default)]
    pub glosaEspecialidad: String,
    #[serde(default)]
    pub glosaSubEspecialidad: String,
    pub urlImagen: String,
    pub urlAccesoAgenda: String,
    pub tieneAgenda: bool,
    #[serde(default)]
    pub idioma: Option<String>,
    pub codigoAreaMedicaSpecified: String,
    pub idSpecified: bool,
    pub tieneAgendaSpecified: bool,
}

#[derive(Serialize)]
pub struct ResultEntry {
    pub id: u32,
    pub doctors: Vec<Doctor>,
}