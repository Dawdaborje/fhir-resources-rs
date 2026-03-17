//! FHIR R5 Communication Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunicationPayload {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Message part content
    #[serde(rename = "contentAttachment")]
    pub content_attachment: Attachment,

    #[serde(rename = "contentReference")]
    pub content_reference: Box<Reference>,

    #[serde(rename = "contentCodeableConcept")]
    pub content_codeable_concept: CodeableConcept,
}

/// A clinical or business level record of information being transmitted or shared; e.g. an alert that was sent to a responsible provider, a public health agency communication to a provider/reporter in...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Communication {
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

    /// Unique identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<String>>,

    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<String>>,

    /// Request fulfilled by this communication
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Part of referenced event (e.g. Communication, Procedure)
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// Reply to
    #[serde(rename = "inResponseTo")]
    pub in_response_to: Option<Vec<Box<Reference>>>,

    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    pub status: String,

    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,

    /// Message category
    pub category: Option<Vec<CodeableConcept>>,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// A channel of communication
    pub medium: Option<Vec<CodeableConcept>>,

    /// Focus of message
    pub subject: Option<Box<Reference>>,

    /// Description of the purpose/content
    pub topic: Option<CodeableConcept>,

    /// Resources that pertain to this communication
    pub about: Option<Vec<Box<Reference>>>,

    /// The Encounter during which this Communication was created
    pub encounter: Option<Box<Reference>>,

    /// When sent
    pub sent: Option<String>,

    /// When received
    pub received: Option<String>,

    /// Who the information is shared with
    pub recipient: Option<Vec<Box<Reference>>>,

    /// Who shares the information
    pub sender: Option<Box<Reference>>,

    /// Indication for message
    pub reason: Option<Vec<CodeableReference>>,

    /// Message payload
    pub payload: Option<Vec<CommunicationPayload>>,

    /// Comments made about the communication
    pub note: Option<Vec<Annotation>>,
}
