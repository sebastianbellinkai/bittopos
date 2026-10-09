use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::resource::{
    normalize_description,
    normalize_name,
    validate_stored_name,
    Resource,
    ResourcePatch,
    ResourceWarning,
};
