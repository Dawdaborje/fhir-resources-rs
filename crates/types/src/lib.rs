//! FHIR Shared Types
//!
//! Common types used across all FHIR versions.

use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// A reference from one resource to another
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference<T> {
    /// Literal reference, Relative, internal or absolute URL
    pub reference: Option<String>,

    /// Type the reference refers to (e.g. "Patient")
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// Logical reference, when literal reference is not known
    pub identifier: Option<String>, // Simplified for now, should be Identifier type

    /// Text alternative for the resource
    pub display: Option<String>,

    /// Phantom data to bind the generic type
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

impl<T> Default for Reference<T> {
    fn default() -> Self {
        Self {
            reference: None,
            r#type: None,
            identifier: None,
            display: None,
            _phantom: PhantomData,
        }
    }
}
