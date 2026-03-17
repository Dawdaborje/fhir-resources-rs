//! FHIR R5 Transport Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Constraints on fulfillment transports
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransportRestriction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// How many times to repeat
    pub repetitions: Option<i32>,

    /// When fulfillment sought
    pub period: Option<Period>,

    /// For whom is fulfillment sought?
    pub recipient: Option<Vec<Box<Reference>>>,
}

/// Information used to perform transport
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransportInput {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Label for the input
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Content to use in performing the transport
    pub value: serde_json::Value,
}

/// Information produced as part of transport
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransportOutput {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Label for output
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Result of output
    pub value: serde_json::Value,
}

/// Record of transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transport {
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

    /// External identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Formal definition of transport
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<String>,

    /// Formal definition of transport
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<String>,

    /// Request fulfilled by this transport
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Requisition or grouper id
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Box<Identifier>>,

    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// in-progress | completed | abandoned | cancelled | planned | entered-in-error
    pub status: Option<String>,

    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,

    /// unknown | proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: String,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// Transport Type
    pub code: Option<CodeableConcept>,

    /// Human-readable explanation of transport
    pub description: Option<String>,

    /// What transport is acting on
    pub focus: Option<Box<Reference>>,

    /// Beneficiary of the Transport
    #[serde(rename = "for")]
    pub r#for: Option<Box<Reference>>,

    /// Healthcare event during which this transport originated
    pub encounter: Option<Box<Reference>>,

    /// Completion time of the event (the occurrence)
    #[serde(rename = "completionTime")]
    pub completion_time: Option<String>,

    /// Transport Creation Date
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<String>,

    /// Transport Last Modified Date
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,

    /// Who is asking for transport to be done
    pub requester: Option<Box<Reference>>,

    /// Requested performer
    #[serde(rename = "performerType")]
    pub performer_type: Option<Vec<CodeableConcept>>,

    /// Responsible individual
    pub owner: Option<Box<Reference>>,

    /// Where transport occurs
    pub location: Option<Box<Reference>>,

    /// Associated insurance coverage
    pub insurance: Option<Vec<Box<Reference>>>,

    /// Comments made about the transport
    pub note: Option<Vec<Annotation>>,

    /// Key events in history of the Transport
    #[serde(rename = "relevantHistory")]
    pub relevant_history: Option<Vec<Box<Reference>>>,

    /// Constraints on fulfillment transports
    pub restriction: Option<TransportRestriction>,

    /// Information used to perform transport
    pub input: Option<Vec<TransportInput>>,

    /// Information produced as part of transport
    pub output: Option<Vec<TransportOutput>>,

    /// The desired location
    #[serde(rename = "requestedLocation")]
    pub requested_location: Box<Reference>,

    /// The entity current location
    #[serde(rename = "currentLocation")]
    pub current_location: Box<Reference>,

    /// Why transport is needed
    pub reason: Option<CodeableReference>,

    /// Parent (or preceding) transport
    pub history: Option<Box<Reference>>,
}
