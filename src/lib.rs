//! Núcleo mínimo de Bittopos.
//!
//! En esta versión, un recurso contiene identidad, nombre y descripción opcional.
//! Cada recurso se guarda en un archivo TOML cuyo nombre es su UUID.

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resource {
    /// Identidad estable. Se serializa primero para facilitar la comprobación del archivo.
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResourcePatch {
    /// None conserva el nombre; Some(value) solicita cambiarlo.
    pub name: Option<String>,
    /// None conserva la descripción; Some(None) la elimina.
    /// Some(Some("")) también elimina la descripción.
    pub description: Option<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceWarning {
    LeadingSpaces { id: Uuid },
}

#[derive(Debug)]
pub enum StoreError {
    Io(io::Error),
    TomlSerialize(toml::ser::Error),
    TomlParse(toml::de::Error),
    InvalidName,
    NotFound(Uuid),
    InvalidFileName(PathBuf),
    IdMismatch { expected: Uuid, found: Uuid },
    IdNotFirstLine(PathBuf),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "error de E/S: {e}"),
            Self::TomlSerialize(e) => write!(f, "no se pudo serializar TOML: {e}"),
            Self::TomlParse(e) => write!(f, "TOML inválido: {e}"),
            Self::InvalidName => write!(f, "el nombre no puede estar vacío ni contener solo espacios"),
            Self::NotFound(id) => write!(f, "no existe el recurso {id}"),
            Self::InvalidFileName(path) => write!(f, "nombre de archivo de recurso inválido: {}", path.display()),
            Self::IdMismatch { expected, found } => write!(f, "el ID del archivo ({expected}) no coincide con el ID del contenido ({found})"),
            Self::IdNotFirstLine(path) => write!(f, "el ID no aparece en la primera línea de {}", path.display()),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<io::Error> for StoreError {
    fn from(value: io::Error) -> Self { Self::Io(value) }
}
impl From<toml::ser::Error> for StoreError {
    fn from(value: toml::ser::Error) -> Self { Self::TomlSerialize(value) }
}
impl From<toml::de::Error> for StoreError {
    fn from(value: toml::de::Error) -> Self { Self::TomlParse(value) }
}

/// Almacén local de recursos. El prototipo presupone un solo proceso escritor.
pub struct ResourceStore {
    root: PathBuf,
}

impl ResourceStore {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self, StoreError> {
        let root = root.into();
        fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path { &self.root }

    pub fn create(&self, name: &str, description: Option<&str>) -> Result<(Resource, Vec<ResourceWarning>), StoreError> {
        let (name, _) = normalize_name(name)?;
        let mut warnings = Vec::new();
        let id = Uuid::now_v7();
        let description = normalize_description(description.map(str::to_owned));
        let resource = Resource { id, name, description };
        self.write_new(&resource)?;
        if resource.name.starts_with(' ') {
            warnings.push(ResourceWarning::LeadingSpaces { id });
        }
        Ok((resource, warnings))
    }

    pub fn get(&self, id: Uuid) -> Result<Resource, StoreError> {
        let path = self.path_for(id);
        if !path.is_file() { return Err(StoreError::NotFound(id)); }
        self.read_file(&path, id)
    }

    pub fn update(&self, id: Uuid, patch: ResourcePatch) -> Result<(Resource, Vec<ResourceWarning>), StoreError> {
        let mut resource = self.get(id)?;
        let mut warnings = Vec::new();

        if let Some(name) = patch.name {
            let (normalized, _) = normalize_name(&name)?;
            if normalized.starts_with(' ') {
                warnings.push(ResourceWarning::LeadingSpaces { id });
            }
            resource.name = normalized;
        }
        if let Some(description) = patch.description {
            resource.description = normalize_description(description);
        }

        self.write_existing(&resource)?;
        Ok((resource, warnings))
    }

    /// Recupera todos los TOML válidos. Los archivos inválidos se omiten y generan advertencias.
    /// Los archivos temporales y otros archivos no terminados en `.toml` se ignoran.
    pub fn recover(&self) -> Result<(Vec<Resource>, Vec<String>), StoreError> {
        let mut resources = Vec::new();
        let mut warnings = Vec::new();

        for entry in fs::read_dir(&self.root)? {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    warnings.push(format!("no se pudo leer una entrada del directorio: {error}"));
                    continue;
                }
            };
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("toml") { continue; }

            let stem = match path.file_stem().and_then(|s| s.to_str()) {
                Some(stem) => stem,
                None => {
                    warnings.push(format!("nombre de archivo inválido: {}", path.display()));
                    continue;
                }
            };
            let expected_id = match Uuid::parse_str(stem) {
                Ok(id) => id,
                Err(_) => {
                    warnings.push(format!("nombre de archivo no contiene un UUID válido: {}", path.display()));
                    continue;
                }
            };

            match self.read_file(&path, expected_id) {
                Ok(resource) => resources.push(resource),
                Err(error) => warnings.push(format!("se omitió {}: {error}", path.display())),
            }
        }

        resources.sort_by_key(|resource| resource.id);
        Ok((resources, warnings))
    }

    fn path_for(&self, id: Uuid) -> PathBuf {
        self.root.join(format!("{id}.toml"))
    }

    fn write_new(&self, resource: &Resource) -> Result<(), StoreError> {
        let path = self.path_for(resource.id);
        let content = toml::to_string(resource)?;
        let mut temp = tempfile::NamedTempFile::new_in(&self.root)?;
        temp.write_all(content.as_bytes())?;
        temp.as_file().sync_all()?;
        temp.persist_noclobber(path).map_err(|error| StoreError::Io(error.error))?;
        Ok(())
    }

    fn write_existing(&self, resource: &Resource) -> Result<(), StoreError> {
        let path = self.path_for(resource.id);
        let content = toml::to_string(resource)?;
        let mut temp = tempfile::NamedTempFile::new_in(&self.root)?;
        temp.write_all(content.as_bytes())?;
        temp.as_file().sync_all()?;
        temp.persist(path).map_err(|error| StoreError::Io(error.error))?;
        Ok(())
    }

    fn read_file(&self, path: &Path, expected_id: Uuid) -> Result<Resource, StoreError> {
        let content = fs::read_to_string(path)?;
        let first_line = content.lines().next().unwrap_or_default();
        if !first_line.starts_with("id = ") {
            return Err(StoreError::IdNotFirstLine(path.to_path_buf()));
        }
        let resource: Resource = toml::from_str(&content)?;
        if resource.id != expected_id {
            return Err(StoreError::IdMismatch { expected: expected_id, found: resource.id });
        }
        validate_stored_name(&resource.name)?;
        Ok(resource)
    }
}

fn normalize_name(name: &str) -> Result<(String, Vec<ResourceWarning>), StoreError> {
    let normalized = name.trim_end_matches(' ').to_owned();
    if normalized.is_empty() { return Err(StoreError::InvalidName); }
    // Los espacios iniciales se conservan por ahora; el llamador emite el aviso.
    Ok((normalized, Vec::new()))
}

fn validate_stored_name(name: &str) -> Result<(), StoreError> {
    if name.is_empty() || name.trim_matches(' ').is_empty() {
        return Err(StoreError::InvalidName);
    }
    Ok(())
}

fn normalize_description(description: Option<String>) -> Option<String> {
    description.filter(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn store() -> (tempfile::TempDir, ResourceStore) {
        let dir = tempfile::tempdir().unwrap();
        let store = ResourceStore::new(dir.path()).unwrap();
        (dir, store)
    }

    #[test]
    fn creates_uuid_v7_resource_and_toml_file() {
        let (_dir, store) = store();
        let (resource, warnings) = store.create("Primer recurso", Some("Contexto")).unwrap();
        assert_eq!(resource.id.get_version_num(), 7);
        assert!(warnings.is_empty());
        let file = fs::read_to_string(store.path_for(resource.id)).unwrap();
        assert!(file.lines().next().unwrap().starts_with("id = "));
        assert!(file.contains("name = \"Primer recurso\""));
    }

    #[test]
    fn same_name_gets_distinct_ids() {
        let (_dir, store) = store();
        let (a, _) = store.create("Igual", None).unwrap();
        let (b, _) = store.create("Igual", None).unwrap();
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn recovers_resource_from_disk() {
        let (_dir, store) = store();
        let (created, _) = store.create("Persistente", Some("Texto")).unwrap();
        let (recovered, warnings) = store.recover().unwrap();
        assert!(warnings.is_empty());
        assert_eq!(recovered, vec![created]);
    }

    #[test]
    fn patch_preserves_omitted_fields_and_identity() {
        let (_dir, store) = store();
        let (created, _) = store.create("Antes", Some("Descripción")).unwrap();
        let (updated, _) = store.update(created.id, ResourcePatch {
            name: Some("Después  ".to_owned()),
            description: None,
        }).unwrap();
        assert_eq!(updated.id, created.id);
        assert_eq!(updated.name, "Después");
        assert_eq!(updated.description.as_deref(), Some("Descripción"));
    }

    #[test]
    fn rejects_blank_names_and_warns_about_leading_space() {
        let (_dir, store) = store();
        assert!(matches!(store.create("   ", None), Err(StoreError::InvalidName)));
        let (resource, warnings) = store.create(" nombre", None).unwrap();
        assert_eq!(resource.name, " nombre");
        assert_eq!(warnings, vec![ResourceWarning::LeadingSpaces { id: resource.id }]);
    }

    #[test]
    fn missing_id_returns_controlled_error() {
        let (_dir, store) = store();
        let missing = Uuid::now_v7();
        assert!(matches!(store.get(missing), Err(StoreError::NotFound(id)) if id == missing));
    }

    #[test]
    fn recovery_skips_corrupt_file_and_continues() {
        let (_dir, store) = store();
        let (valid, _) = store.create("Válido", None).unwrap();
        let corrupt_id = Uuid::now_v7();
        fs::write(store.path_for(corrupt_id), "esto no es TOML válido = [").unwrap();
        let (recovered, warnings) = store.recover().unwrap();
        assert_eq!(recovered, vec![valid]);
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn recovery_rejects_filename_content_id_mismatch() {
        let (_dir, store) = store();
        let (resource, _) = store.create("Original", None).unwrap();
        let other_id = Uuid::now_v7();
        fs::rename(store.path_for(resource.id), store.path_for(other_id)).unwrap();
        let (recovered, warnings) = store.recover().unwrap();
        assert!(recovered.is_empty());
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("no coincide"));
    }

    #[test]
    fn empty_description_clears_it() {
        let (_dir, store) = store();
        let (resource, _) = store.create("Recurso", Some("Texto")).unwrap();
        let (updated, _) = store.update(resource.id, ResourcePatch {
            name: None,
            description: Some(Some(String::new())),
        }).unwrap();
        assert_eq!(updated.description, None);
    }
}
