//! FHIR R4 NamingSystem Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Unique identifiers used for system
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamingSystemUniqueId {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// oid | uuid | uri | other
    #[serde(rename = "type")]
    pub r#type: String,

    /// The unique identifier
    pub value: String,

    /// Is this the id that should be used for this type
    pub preferred: Option<bool>,

    /// Notes about identifier usage
    pub comment: Option<String>,

    /// When is identifier valid?
    pub period: Option<Period>,
}

/// A curated namespace that issues unique symbols within that namespace for the identification of concepts, people, devices, etc. Represents a "System" used within the Identifier and Coding data types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamingSystem {
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

    /// Name for this naming system (computer friendly)
    pub name: String,

    /// draft | active | retired | unknown
    pub status: String,

    /// codesystem | identifier | root
    pub kind: String,

    /// Date last changed
    pub date: String,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Who maintains system namespace?
    pub responsible: Option<String>,

    /// e.g. driver, provider, patient, bank etc.
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Natural language description of the naming system
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for naming system (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// How/where is it used
    pub usage: Option<String>,

    /// Unique identifiers used for system
    #[serde(rename = "uniqueId")]
    pub unique_id: Vec<NamingSystemUniqueId>,
}
