//! FHIR R4B Provenance Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Actor involved
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceAgent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// How the agent participated
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// What the agents role was
    pub role: Option<Vec<CodeableConcept>>,

    /// Who participated
    pub who: Box<Reference>,

    /// Who the agent is representing
    #[serde(rename = "onBehalfOf")]
    pub on_behalf_of: Option<Box<Reference>>,
}

/// An entity used in this activity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceEntity {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// derivation | revision | quotation | source | removal
    pub role: String,

    /// Identity of entity
    pub what: Box<Reference>,

    /// Entity is attributed to this agent
    pub agent: Option<Vec<ProvenanceAgent>>,
}

/// Provenance of a resource is a record that describes entities and processes involved in producing and delivering or otherwise influencing that resource. Provenance provides a critical foundation for...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Provenance {
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

    /// Target Reference(s) (usually version specific)
    pub target: Vec<Box<Reference>>,

    /// When the activity occurred
    #[serde(rename = "occurredPeriod")]
    pub occurred_period: Option<Period>,

    #[serde(rename = "occurredDateTime")]
    pub occurred_date_time: Option<String>,

    /// When the activity was recorded / updated
    pub recorded: String,

    /// Policy or plan the activity was defined by
    pub policy: Option<Vec<String>>,

    /// Where the activity occurred, if relevant
    pub location: Option<Box<Reference>>,

    /// Reason the activity is occurring
    pub reason: Option<Vec<CodeableConcept>>,

    /// Activity that occurred
    pub activity: Option<CodeableConcept>,

    /// Actor involved
    pub agent: Vec<ProvenanceAgent>,

    /// An entity used in this activity
    pub entity: Option<Vec<ProvenanceEntity>>,

    /// Signature on target
    pub signature: Option<Vec<Signature>>,
}
