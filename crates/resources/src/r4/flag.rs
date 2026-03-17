//! FHIR R4 Flag Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Prospective warnings of potential issues when providing care to the patient.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag {
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

    /// Business identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | inactive | entered-in-error
    pub status: String,

    /// Clinical, administrative, etc.
    pub category: Option<Vec<CodeableConcept>>,

    /// Coded or textual message to display to user
    pub code: CodeableConcept,

    /// Who/What is flag about?
    pub subject: Box<Reference>,

    /// Time period when flag is active
    pub period: Option<Period>,

    /// Alert relevant during encounter
    pub encounter: Option<Box<Reference>>,

    /// Flag creator
    pub author: Option<Box<Reference>>,
}
