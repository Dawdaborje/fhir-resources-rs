//! FHIR R5 ServiceRequest Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Additional order information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRequestOrderDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The context of the order details by reference
    #[serde(rename = "parameterFocus")]
    pub parameter_focus: Option<CodeableReference>,

    /// The parameter details for the service being requested
    pub parameter: Vec<ServiceRequestOrderDetailParameter>,
}

/// The parameter details for the service being requested
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRequestOrderDetailParameter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The detail of the order being requested
    pub code: CodeableConcept,

    /// The value for the order detail
    pub value: serde_json::Value,
}

/// Patient or consumer-oriented instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRequestPatientInstruction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Patient or consumer-oriented instructions
    pub instruction: Option<serde_json::Value>,
}

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

    /// proposal | plan | directive | order +
    pub intent: String,

    /// Classification of service
    pub category: Option<Vec<CodeableConcept>>,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// True if service/procedure should not be performed
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<bool>,

    /// What is being requested/ordered
    pub code: Option<CodeableReference>,

    /// Additional order information
    #[serde(rename = "orderDetail")]
    pub order_detail: Option<Vec<ServiceRequestOrderDetail>>,

    /// Service amount
    pub quantity: Option<serde_json::Value>,

    /// Individual or Entity the service is ordered for
    pub subject: Box<Reference>,

    /// What the service request is about, when it is not about the subject of record
    pub focus: Option<Vec<Box<Reference>>>,

    /// Encounter in which the request was created
    pub encounter: Option<Box<Reference>>,

    /// When service should occur
    pub occurrence: Option<serde_json::Value>,

    /// Preconditions for service
    #[serde(rename = "asNeeded")]
    pub as_needed: Option<serde_json::Value>,

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
    pub location: Option<Vec<CodeableReference>>,

    /// Explanation/Justification for procedure or service
    pub reason: Option<Vec<CodeableReference>>,

    /// Associated insurance coverage
    pub insurance: Option<Vec<Box<Reference>>>,

    /// Additional clinical information
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<CodeableReference>>,

    /// Procedure Samples
    pub specimen: Option<Vec<Box<Reference>>>,

    /// Coded location on Body
    #[serde(rename = "bodySite")]
    pub body_site: Option<Vec<CodeableConcept>>,

    /// BodyStructure-based location on the body
    #[serde(rename = "bodyStructure")]
    pub body_structure: Option<Box<Reference>>,

    /// Comments
    pub note: Option<Vec<Annotation>>,

    /// Patient or consumer-oriented instructions
    #[serde(rename = "patientInstruction")]
    pub patient_instruction: Option<Vec<ServiceRequestPatientInstruction>>,

    /// Request provenance
    #[serde(rename = "relevantHistory")]
    pub relevant_history: Option<Vec<Box<Reference>>>,
}
