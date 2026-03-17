//! FHIR R4 EnrollmentRequest Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// This resource provides the insurance enrollment details to the insurer regarding a specified coverage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentRequest {
    /// The type of resource
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    /// Logical id of this artifact
    pub id: Option<String>,

    /// Metadata about the resource
    pub meta: Option<Meta>,

    /// A set of rules under which this content was created
    #[serde(rename = "implicitRules")]
    pub implicit_rules: Option<String>,

    /// Language of the resource content
    pub language: Option<String>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<Narrative>,

    /// Contained, inline Resources
    pub contained: Option<Vec<serde_json::Value>>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Business Identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | cancelled | draft | entered-in-error
    pub status: Option<String>,

    /// Creation date
    pub created: Option<String>,

    /// Target
    pub insurer: Option<Box<Reference>>,

    /// Responsible practitioner
    pub provider: Option<Box<Reference>>,

    /// The subject to be enrolled
    pub candidate: Option<Box<Reference>>,

    /// Insurance information
    pub coverage: Option<Box<Reference>>,
}
