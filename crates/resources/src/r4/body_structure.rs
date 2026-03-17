//! FHIR R4 BodyStructure Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Record details about an anatomical structure. This resource may be used when a coded concept does not provide the necessary detail needed for the use case.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyStructure {
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

    /// Bodystructure identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Whether this record is in active use
    pub active: Option<bool>,

    /// Kind of Structure
    pub morphology: Option<CodeableConcept>,

    /// Body site
    pub location: Option<CodeableConcept>,

    /// Body site modifier
    #[serde(rename = "locationQualifier")]
    pub location_qualifier: Option<Vec<CodeableConcept>>,

    /// Text description
    pub description: Option<String>,

    /// Attached images
    pub image: Option<Vec<Attachment>>,

    /// Who this is about
    pub patient: Box<Reference>,
}
