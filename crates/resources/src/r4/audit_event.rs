//! FHIR R4 AuditEvent Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Actor involved in the event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEventAgent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// How agent participated
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Agent role in the event
    pub role: Option<Vec<CodeableConcept>>,

    /// Identifier of who
    pub who: Option<Box<Reference>>,

    /// Alternative User identity
    #[serde(rename = "altId")]
    pub alt_id: Option<String>,

    /// Human friendly name for the agent
    pub name: Option<String>,

    /// Whether user is initiator
    pub requestor: bool,

    /// Where
    pub location: Option<Box<Reference>>,

    /// Policy that authorized event
    pub policy: Option<Vec<String>>,

    /// Type of media
    pub media: Option<Coding>,

    /// Logical network location for application activity
    pub network: Option<AuditEventAgentNetwork>,

    /// Reason given for this user
    #[serde(rename = "purposeOfUse")]
    pub purpose_of_use: Option<Vec<CodeableConcept>>,
}

/// Logical network location for application activity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEventAgentNetwork {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Identifier for the network access point of the user device
    pub address: Option<String>,

    /// The type of network access point
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

/// Audit Event Reporter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEventSource {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Logical source location within the enterprise
    pub site: Option<String>,

    /// The identity of source detecting the event
    pub observer: Box<Reference>,

    /// The type of source where event originated
    #[serde(rename = "type")]
    pub r#type: Option<Vec<Coding>>,
}

/// Data or objects used
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEventEntity {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Specific instance of resource
    pub what: Option<Box<Reference>>,

    /// Type of entity involved
    #[serde(rename = "type")]
    pub r#type: Option<Coding>,

    /// What role the entity played
    pub role: Option<Coding>,

    /// Life-cycle stage for the entity
    pub lifecycle: Option<Coding>,

    /// Security labels on the entity
    #[serde(rename = "securityLabel")]
    pub security_label: Option<Vec<Coding>>,

    /// Descriptor for entity
    pub name: Option<String>,

    /// Descriptive text
    pub description: Option<String>,

    /// Query parameters
    pub query: Option<String>,

    /// Additional Information about the entity
    pub detail: Option<Vec<AuditEventEntityDetail>>,
}

/// Additional Information about the entity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEventEntityDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name of the property
    #[serde(rename = "type")]
    pub r#type: String,

    /// Property value
    pub value: serde_json::Value,
}

/// A record of an event made for purposes of maintaining a security log. Typical uses include detection of intrusion attempts and monitoring for inappropriate usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEvent {
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

    /// Type/identifier of event
    #[serde(rename = "type")]
    pub r#type: Coding,

    /// More specific type/id for the event
    pub subtype: Option<Vec<Coding>>,

    /// Type of action performed during the event
    pub action: Option<String>,

    /// When the activity occurred
    pub period: Option<Period>,

    /// Time when the event was recorded
    pub recorded: String,

    /// Whether the event succeeded or failed
    pub outcome: Option<String>,

    /// Description of the event outcome
    #[serde(rename = "outcomeDesc")]
    pub outcome_desc: Option<String>,

    /// The purposeOfUse of the event
    #[serde(rename = "purposeOfEvent")]
    pub purpose_of_event: Option<Vec<CodeableConcept>>,

    /// Actor involved in the event
    pub agent: Vec<AuditEventAgent>,

    /// Audit Event Reporter
    pub source: AuditEventSource,

    /// Data or objects used
    pub entity: Option<Vec<AuditEventEntity>>,
}
