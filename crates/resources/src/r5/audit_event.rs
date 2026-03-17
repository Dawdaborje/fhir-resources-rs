//! FHIR R5 AuditEvent Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Whether the event succeeded or failed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEventOutcome {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Whether the event succeeded or failed
    pub code: Coding,

    /// Additional outcome detail
    pub detail: Option<Vec<CodeableConcept>>,
}

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
    pub who: Box<Reference>,

    /// Whether user is initiator
    pub requestor: Option<bool>,

    /// The agent location when the event occurred
    pub location: Option<Box<Reference>>,

    /// Policy that authorized the agent participation in the event
    pub policy: Option<Vec<String>>,

    /// This agent network location for the activity
    #[serde(rename = "networkReference")]
    pub network_reference: Option<Box<Reference>>,

    #[serde(rename = "networkUri")]
    pub network_uri: Option<String>,

    #[serde(rename = "networkString")]
    pub network_string: Option<String>,

    /// Allowable authorization for this agent
    pub authorization: Option<Vec<CodeableConcept>>,
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
    pub site: Option<Box<Reference>>,

    /// The identity of source detecting the event
    pub observer: Box<Reference>,

    /// The type of source where event originated
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,
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

    /// What role the entity played
    pub role: Option<CodeableConcept>,

    /// Security labels on the entity
    #[serde(rename = "securityLabel")]
    pub security_label: Option<Vec<CodeableConcept>>,

    /// Query parameters
    pub query: Option<String>,

    /// Additional Information about the entity
    pub detail: Option<Vec<AuditEventEntityDetail>>,

    /// Entity is attributed to this agent
    pub agent: Option<Vec<AuditEventAgent>>,
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
    pub r#type: CodeableConcept,

    /// Property value
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,

    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,

    #[serde(rename = "valueString")]
    pub value_string: String,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: bool,

    #[serde(rename = "valueInteger")]
    pub value_integer: i32,

    #[serde(rename = "valueRange")]
    pub value_range: Range,

    #[serde(rename = "valueRatio")]
    pub value_ratio: Ratio,

    #[serde(rename = "valueTime")]
    pub value_time: String,

    #[serde(rename = "valueDateTime")]
    pub value_date_time: String,

    #[serde(rename = "valuePeriod")]
    pub value_period: Period,

    #[serde(rename = "valueBase64Binary")]
    pub value_base64binary: String,
}

/// A record of an event relevant for purposes such as operations, privacy, security, maintenance, and performance analysis.
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
    pub category: Option<Vec<CodeableConcept>>,

    /// Specific type of event
    pub code: CodeableConcept,

    /// Type of action performed during the event
    pub action: Option<String>,

    /// emergency | alert | critical | error | warning | notice | informational | debug
    pub severity: Option<String>,

    /// When the activity occurred
    #[serde(rename = "occurredPeriod")]
    pub occurred_period: Option<Period>,

    #[serde(rename = "occurredDateTime")]
    pub occurred_date_time: Option<String>,

    /// Time when the event was recorded
    pub recorded: String,

    /// Whether the event succeeded or failed
    pub outcome: Option<AuditEventOutcome>,

    /// Authorization related to the event
    pub authorization: Option<Vec<CodeableConcept>>,

    /// Workflow authorization within which this event occurred
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// The patient is the subject of the data used/created/updated/deleted during the activity
    pub patient: Option<Box<Reference>>,

    /// Encounter within which this event occurred or which the event is tightly associated
    pub encounter: Option<Box<Reference>>,

    /// Actor involved in the event
    pub agent: Vec<AuditEventAgent>,

    /// Audit Event Reporter
    pub source: AuditEventSource,

    /// Data or objects used
    pub entity: Option<Vec<AuditEventEntity>>,
}
