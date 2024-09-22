use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct DoctorOverview {
    pub id: u64,
    pub nombres: String,
    pub apellidoMaterno: Option<String>, /* Only 1 case... */
    pub apellidoPaterno: Option<String>,
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

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct DoctorProfile {
    #[serde(default)]
    pub ppn: Option<u64>,
    #[serde(default)]
    pub email: Option<String>,
    pub gender: Option<String>,
    pub fullName: String,
    pub name: String,
    pub lastName: String,
    pub image: String,
    #[serde(default)]
    pub areasMedicas: Vec<String>,
    #[serde(default)]
    pub areasInteres: Vec<String>,
    #[serde(default)]
    pub tieneTelemedicina: bool,
    #[serde(default)]
    pub queTrata: Vec<String>,
    #[serde(default)]
    pub educacion: Vec<Education>,
    #[serde(default)]
    pub docencia: Vec<String>,
    #[serde(default)]
    pub especialidades: Vec<Specialty>,
    #[serde(default)]
    pub subEspecialidades: Vec<String>,
    #[serde(default)]
    pub idiomas: Vec<String>,
    #[serde(default)]
    pub atencionClinica: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Education {
    pub annoIngreso: Option<u32>,
    pub annoEgreso: Option<u32>,
    pub nombre: String,
    pub institucion: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Specialty {
    pub especialidadId: Option<u32>,
    pub nombres: String,
}

#[derive(Serialize)]
pub struct ResultEntry {
    pub id: Option<u32>,
    pub doctors: Vec<DoctorProfile>,
}
