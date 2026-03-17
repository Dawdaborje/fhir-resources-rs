//! FHIR R4B Procedure Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// The people who performed the procedure
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

    /// The reference to the practitioner
    pub actor: Box<Reference>,

    /// Organization the device or practitioner was acting for
    #[serde(rename = "onBehalfOf")]
    pub on_behalf_of: Option<Box<Reference>>,
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

/// An action that is or was performed on or for a patient. This can be a physical intervention like an operation, or less invasive like long term services, counseling, or hypnotherapy.
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
    pub category: Option<CodeableConcept>,

    /// Identification of the procedure
    pub code: Option<CodeableConcept>,

    /// Who the procedure was performed on
    pub subject: Box<Reference>,

    /// Encounter created as part of
    pub encounter: Option<Box<Reference>>,

    /// When the procedure was performed
    #[serde(rename = "performedDateTime")]
    pub performed_date_time: Option<String>,

    #[serde(rename = "performedPeriod")]
    pub performed_period: Option<Period>,

    #[serde(rename = "performedString")]
    pub performed_string: Option<String>,

    #[serde(rename = "performedAge")]
    pub performed_age: Option<Age>,

    #[serde(rename = "performedRange")]
    pub performed_range: Option<Range>,

    /// Who recorded the procedure
    pub recorder: Option<Box<Reference>>,

    /// Person who asserts this procedure
    pub asserter: Option<Box<Reference>>,

    /// The people who performed the procedure
    pub performer: Option<Vec<ProcedurePerformer>>,

    /// Where the procedure happened
    pub location: Option<Box<Reference>>,

    /// Coded reason procedure performed
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// The justification that the procedure was performed
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// Target body sites
    #[serde(rename = "bodySite")]
    pub body_site: Option<Vec<CodeableConcept>>,

    /// The result of procedure
    pub outcome: Option<CodeableConcept>,

    /// Any report resulting from the procedure
    pub report: Option<Vec<Box<Reference>>>,

    /// Complication following the procedure
    pub complication: Option<Vec<CodeableConcept>>,

    /// A condition that is a result of the procedure
    #[serde(rename = "complicationDetail")]
    pub complication_detail: Option<Vec<Box<Reference>>>,

    /// Instructions for follow up
    #[serde(rename = "followUp")]
    pub follow_up: Option<Vec<CodeableConcept>>,

    /// Additional information about the procedure
    pub note: Option<Vec<Annotation>>,

    /// Manipulated, implanted, or removed device
    #[serde(rename = "focalDevice")]
    pub focal_device: Option<Vec<ProcedureFocalDevice>>,

    /// Items used during procedure
    #[serde(rename = "usedReference")]
    pub used_reference: Option<Vec<Box<Reference>>>,

    /// Coded items used during the procedure
    #[serde(rename = "usedCode")]
    pub used_code: Option<Vec<CodeableConcept>>,
}
