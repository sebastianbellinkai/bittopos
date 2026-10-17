//! Núcleo mínimo de Bittopos.
//!
//! Los recursos se guardan en archivos TOML individuales. La estructura se guarda
//! en un archivo JSON y contiene instancias con identidad propia.

mod application;
mod lifecycle;
mod mask_operations;
mod model_operations;
mod navigation;
mod relations;
mod resource;
mod semantic_masks;
mod store;
mod workspace;
mod workspace_model;

pub use application::{
    create_instance_at_current_location, prepare_workspace, remove_instance_and_update_navigation,
    ApplicationError, PreparedWorkspace,
};
pub use lifecycle::{
    create_instance, create_instance_under, load_workspace, remove_instance, LifecycleError,
};
pub use mask_operations::{
    create_snapshot, edit_instance_component, edit_instance_concept, edit_shared_component,
    edit_shared_concept, remove_mask, resolve_instance, MaskOperationError,
};
pub use model_operations::children_of;
pub use navigation::{NavigationError, NavigationState, NavigationStore};
pub use relations::{
    create_relation, is_relation, relation_endpoints, RelationEndpoints, RelationError,
};
pub use resource::{
    create_reference, is_reference, reference_target, Info, ReferenceError, Resource,
    ResourcePatch, ResourceWarning,
};
pub use semantic_masks::{
    resolve as resolve_semantics, Component, ComponentMerge, Concept, ElementAddress,
    InstanceSemantics, Mask, MaskError, MaskMode, ReplaceComponent, ResourceDefinition,
    Segment as SemanticSegment, SegmentRole,
};
pub use store::{ResourceStore, StoreError};
pub use workspace::{WorkspaceError, WorkspaceStore};
pub use workspace_model::{Content, Instance, Semantic, Source, Workspace, WorkspaceModelError};
