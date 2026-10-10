//! Núcleo mínimo de Bittopos.
//!
//! En esta versión, un recurso contiene identidad, nombre y descripción opcional.
//! Cada recurso se guarda en un archivo TOML cuyo nombre es su UUID.

mod resource;
mod store;

pub use resource::{Info, Resource, ResourcePatch, ResourceWarning};
pub use store::{ResourceStore, StoreError};
