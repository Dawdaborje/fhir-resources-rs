//! FHIR R5 DeviceRequest Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Device details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequestParameter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Device detail
    pub code: Option<CodeableConcept>,

    /// Value of detail
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,

    #[serde(rename = "valueRange")]
    pub value_range: Option<Range>,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<bool>,
}

/// Represents a request a device to be provided to a specific patient. The device may be an implantable device to be subsequently implanted, or an external assistive device, such as a walker, to be de...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequest {
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

    /// External Request identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<String>>,

    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<String>>,

    /// What request fulfills
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// What request replaces
    pub replaces: Option<Vec<Box<Reference>>>,

    /// Identifier of composite request
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Box<Identifier>>,

    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: Option<String>,

    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: String,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// True if the request is to stop or not to start using the device
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<bool>,

    /// Device requested
    pub code: CodeableReference,

    /// Quantity of devices to supply
    pub quantity: Option<i32>,

    /// Device details
    pub parameter: Option<Vec<DeviceRequestParameter>>,

    /// Focus of request
    pub subject: Box<Reference>,

    /// Encounter motivating request
    pub encounter: Option<Box<Reference>>,

    /// Desired time or schedule for use
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<String>,

    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,

    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,

    /// When recorded
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<String>,

    /// Who/what submitted the device request
    pub requester: Option<Box<Reference>>,

    /// Requested Filler
    pub performer: Option<CodeableReference>,

    /// Coded/Linked Reason for request
    pub reason: Option<Vec<CodeableReference>>,

    /// PRN status of request
    #[serde(rename = "asNeeded")]
    pub as_needed: Option<bool>,

    /// Device usage reason
    #[serde(rename = "asNeededFor")]
    pub as_needed_for: Option<CodeableConcept>,

    /// Associated insurance coverage
    pub insurance: Option<Vec<Box<Reference>>>,

    /// Additional clinical information
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<Box<Reference>>>,

    /// Notes or comments
    pub note: Option<Vec<Annotation>>,

    /// Request provenance
    #[serde(rename = "relevantHistory")]
    pub relevant_history: Option<Vec<Box<Reference>>>,
}
