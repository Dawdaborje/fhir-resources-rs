//! FHIR R4B ServiceRequest Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// A record of a request for service such as diagnostic investigations, treatments, or operations to be performed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRequest {
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

    /// Identifiers assigned to this order
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

    /// Composite Request ID
    pub requisition: Option<Box<Identifier>>,

    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: String,

    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: String,

    /// Classification of service
    pub category: Option<Vec<CodeableConcept>>,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// True if service/procedure should not be performed
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<bool>,

    /// What is being requested/ordered
    pub code: Option<CodeableConcept>,

    /// Additional order information
    #[serde(rename = "orderDetail")]
    pub order_detail: Option<Vec<CodeableConcept>>,

    /// Service amount
    #[serde(rename = "quantityQuantity")]
    pub quantity_quantity: Option<Quantity>,

    #[serde(rename = "quantityRatio")]
    pub quantity_ratio: Option<Ratio>,

    #[serde(rename = "quantityRange")]
    pub quantity_range: Option<Range>,

    /// Individual or Entity the service is ordered for
    pub subject: Box<Reference>,

    /// Encounter in which the request was created
    pub encounter: Option<Box<Reference>>,

    /// When service should occur
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<String>,

    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,

    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,

    /// Preconditions for service
    #[serde(rename = "asNeededBoolean")]
    pub as_needed_boolean: Option<bool>,

    #[serde(rename = "asNeededCodeableConcept")]
    pub as_needed_codeable_concept: Option<CodeableConcept>,

    /// Date request signed
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<String>,

    /// Who/what is requesting service
    pub requester: Option<Box<Reference>>,

    /// Performer role
    #[serde(rename = "performerType")]
    pub performer_type: Option<CodeableConcept>,

    /// Requested performer
    pub performer: Option<Vec<Box<Reference>>>,

    /// Requested location
    #[serde(rename = "locationCode")]
    pub location_code: Option<Vec<CodeableConcept>>,

    /// Requested location
    #[serde(rename = "locationReference")]
    pub location_reference: Option<Vec<Box<Reference>>>,

    /// Explanation/Justification for procedure or service
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Explanation/Justification for service or service
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// Associated insurance coverage
    pub insurance: Option<Vec<Box<Reference>>>,

    /// Additional clinical information
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<Box<Reference>>>,

    /// Procedure Samples
    pub specimen: Option<Vec<Box<Reference>>>,

    /// Location on Body
    #[serde(rename = "bodySite")]
    pub body_site: Option<Vec<CodeableConcept>>,

    /// Comments
    pub note: Option<Vec<Annotation>>,

    /// Patient or consumer-oriented instructions
    #[serde(rename = "patientInstruction")]
    pub patient_instruction: Option<String>,

    /// Request provenance
    #[serde(rename = "relevantHistory")]
    pub relevant_history: Option<Vec<Box<Reference>>>,
}
