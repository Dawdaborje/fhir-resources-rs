//! FHIR R5 Procedure Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Who performed the procedure and what they did
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcedurePerformer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of performance
    pub function: Option<CodeableConcept>,

    /// Who performed the procedure
    pub actor: Box<Reference>,

    /// Organization the device or practitioner was acting for
    #[serde(rename = "onBehalfOf")]
    pub on_behalf_of: Option<Box<Reference>>,

    /// When the performer performed the procedure
    pub period: Option<Period>,
}

/// Manipulated, implanted, or removed device
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcedureFocalDevice {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Kind of change to device
    pub action: Option<CodeableConcept>,

    /// Device that was changed
    pub manipulated: Box<Reference>,
}

/// An action that is or was performed on or for a patient, practitioner, device, organization, or location. For example, this can be a physical intervention on a patient like an operation, or less inv...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedure {
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

    /// External Identifiers for this procedure
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<String>>,

    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<String>>,

    /// A request for this procedure
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    pub status: String,

    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,

    /// Classification of the procedure
    pub category: Option<Vec<CodeableConcept>>,

    /// Identification of the procedure
    pub code: Option<CodeableConcept>,

    /// Individual or entity the procedure was performed on
    pub subject: Box<Reference>,

    /// Who is the target of the procedure when it is not the subject of record only
    pub focus: Option<Box<Reference>>,

    /// The Encounter during which this Procedure was created
    pub encounter: Option<Box<Reference>>,

    /// When the procedure occurred or is occurring
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<String>,

    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,

    #[serde(rename = "occurrenceString")]
    pub occurrence_string: Option<String>,

    #[serde(rename = "occurrenceAge")]
    pub occurrence_age: Option<Age>,

    #[serde(rename = "occurrenceRange")]
    pub occurrence_range: Option<Range>,

    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,

    /// When the procedure was first captured in the subject's record
    pub recorded: Option<String>,

    /// Who recorded the procedure
    pub recorder: Option<Box<Reference>>,

    /// Reported rather than primary record
    #[serde(rename = "reportedBoolean")]
    pub reported_boolean: Option<bool>,

    #[serde(rename = "reportedReference")]
    pub reported_reference: Option<Box<Reference>>,

    /// Who performed the procedure and what they did
    pub performer: Option<Vec<ProcedurePerformer>>,

    /// Where the procedure happened
    pub location: Option<Box<Reference>>,

    /// The justification that the procedure was performed
    pub reason: Option<Vec<CodeableReference>>,

    /// Target body sites
    #[serde(rename = "bodySite")]
    pub body_site: Option<Vec<CodeableConcept>>,

    /// The result of procedure
    pub outcome: Option<CodeableConcept>,

    /// Any report resulting from the procedure
    pub report: Option<Vec<Box<Reference>>>,

    /// Complication following the procedure
    pub complication: Option<Vec<CodeableReference>>,

    /// Instructions for follow up
    #[serde(rename = "followUp")]
    pub follow_up: Option<Vec<CodeableConcept>>,

    /// Additional information about the procedure
    pub note: Option<Vec<Annotation>>,

    /// Manipulated, implanted, or removed device
    #[serde(rename = "focalDevice")]
    pub focal_device: Option<Vec<ProcedureFocalDevice>>,

    /// Items used during procedure
    pub used: Option<Vec<CodeableReference>>,

    /// Extra information relevant to the procedure
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<Box<Reference>>>,
}
