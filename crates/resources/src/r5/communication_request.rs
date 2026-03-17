//! FHIR R5 CommunicationRequest Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Message payload
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunicationRequestPayload {
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

/// A request to convey information; e.g. the CDS system proposes that an alert be sent to a responsible provider, the CDS system proposes that the public health agency be notified about a reportable c...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunicationRequest {
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

    /// Fulfills plan or proposal
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Request(s) replaced by this request
    pub replaces: Option<Vec<Box<Reference>>>,

    /// Composite request this is part of
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Box<Identifier>>,

    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: String,

    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,

    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: String,

    /// Message category
    pub category: Option<Vec<CodeableConcept>>,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// True if request is prohibiting action
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<bool>,

    /// A channel of communication
    pub medium: Option<Vec<CodeableConcept>>,

    /// Focus of message
    pub subject: Option<Box<Reference>>,

    /// Resources that pertain to this communication request
    pub about: Option<Vec<Box<Reference>>>,

    /// The Encounter during which this CommunicationRequest was created
    pub encounter: Option<Box<Reference>>,

    /// Message payload
    pub payload: Option<Vec<CommunicationRequestPayload>>,

    /// When scheduled
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<String>,

    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,

    /// When request transitioned to being actionable
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<String>,

    /// Who asks for the information to be shared
    pub requester: Option<Box<Reference>>,

    /// Who to share the information with
    pub recipient: Option<Vec<Box<Reference>>>,

    /// Who should share the information
    #[serde(rename = "informationProvider")]
    pub information_provider: Option<Vec<Box<Reference>>>,

    /// Why is communication needed?
    pub reason: Option<Vec<CodeableReference>>,

    /// Comments made about communication request
    pub note: Option<Vec<Annotation>>,
}
