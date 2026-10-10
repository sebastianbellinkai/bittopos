use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::store::StoreError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resource {
    /// Identidad estable del recurso.
    /// Se serializa primero para facilitar la comprobación del archivo.
    pub id: Uuid,
    pub info: Info,
}

/// Información descriptiva del recurso.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Info {
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,

    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResourcePatch {
    /// None conserva el título; Some(value) solicita cambiarlo.
    pub title: Option<String>,

    /// None conserva la descripción; Some(None) la elimina.
    /// Some(Some("")) también elimina la descripción.
    pub description: Option<Option<String>>,

    /// None conserva las etiquetas; Some(tags) reemplaza la lista.
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceWarning {
    LeadingSpaces { id: Uuid },
}

pub(crate) fn normalize_name(name: &str) -> Result<(String, Vec<ResourceWarning>), StoreError> {
    let normalized = name.trim_end_matches(' ').to_owned();

    if normalized.is_empty() {
        return Err(StoreError::InvalidName);
    }

    // Los espacios iniciales se conservan por ahora.
    Ok((normalized, Vec::new()))
}

pub(crate) fn validate_stored_name(name: &str) -> Result<(), StoreError> {
    if name.is_empty() || name.trim_matches(' ').is_empty() {
        return Err(StoreError::InvalidName);
    }

    Ok(())
}

pub(crate) fn normalize_description(description: Option<String>) -> Option<String> {
    description.filter(|value| !value.is_empty())
}
