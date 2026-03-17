//! FHIR R4B MessageHeader Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Message destination application(s)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeaderDestination {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name of system
    pub name: Option<String>,

    /// Particular delivery destination within the destination
    pub target: Option<Box<Reference>>,

    /// Actual destination address or id
    pub endpoint: String,

    /// Intended "real-world" recipient for the data
    pub receiver: Option<Box<Reference>>,
}

/// Message source application
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeaderSource {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name of system
    pub name: Option<String>,

    /// Name of software running the system
    pub software: Option<String>,

    /// Version of software running
    pub version: Option<String>,

    /// Human contact for problems
    pub contact: Option<ContactPoint>,

    /// Actual message source address or id
    pub endpoint: String,
}

/// If this is a reply to prior message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeaderResponse {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Id of original message
    pub identifier: String,

    /// ok | transient-error | fatal-error
    pub code: String,

    /// Specific list of hints/warnings/errors
    pub details: Option<Box<Reference>>,
}

/// The header for a message exchange that is either requesting or responding to an action. The reference(s) that are the subject of the action as well as other information related to the action are ty...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeader {
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

    /// Code for the event this message represents or link to event definition
    pub event: serde_json::Value,

    /// Message destination application(s)
    pub destination: Option<Vec<MessageHeaderDestination>>,

    /// Real world sender of the message
    pub sender: Option<Box<Reference>>,

    /// The source of the data entry
    pub enterer: Option<Box<Reference>>,

    /// The source of the decision
    pub author: Option<Box<Reference>>,

    /// Message source application
    pub source: MessageHeaderSource,

    /// Final responsibility for event
    pub responsible: Option<Box<Reference>>,

    /// Cause of event
    pub reason: Option<CodeableConcept>,

    /// If this is a reply to prior message
    pub response: Option<MessageHeaderResponse>,

    /// The actual content of the message
    pub focus: Option<Vec<Box<Reference>>>,

    /// Link to the definition for this message
    pub definition: Option<String>,
}
