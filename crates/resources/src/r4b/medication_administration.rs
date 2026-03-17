//! FHIR R4B MedicationAdministration Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Who performed the medication administration and what they did
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationAdministrationPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of performance
    pub function: Option<CodeableConcept>,

    /// Who performed the medication administration
    pub actor: Box<Reference>,
}

/// Details of how medication was taken
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationAdministrationDosage {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Free text dosage instructions e.g. SIG
    pub text: Option<String>,

    /// Body site administered to
    pub site: Option<CodeableConcept>,

    /// Path of substance into body
    pub route: Option<CodeableConcept>,

    /// How drug was administered
    pub method: Option<CodeableConcept>,

    /// Amount of medication per dose
    pub dose: Option<Quantity>,

    /// Dose quantity per unit of time
    #[serde(rename = "rateRatio")]
    pub rate_ratio: Option<Ratio>,

    #[serde(rename = "rateQuantity")]
    pub rate_quantity: Option<Quantity>,
}

/// Describes the event of a patient consuming or otherwise being administered a medication. This may be as simple as swallowing a tablet or it may be a long running infusion. Related resources tie thi...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationAdministration {
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

    /// Instantiates protocol or definition
    pub instantiates: Option<Vec<String>>,

    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// in-progress | not-done | on-hold | completed | entered-in-error | stopped | unknown
    pub status: String,

    /// Reason administration not performed
    #[serde(rename = "statusReason")]
    pub status_reason: Option<Vec<CodeableConcept>>,

    /// Type of medication usage
    pub category: Option<CodeableConcept>,

    /// What was administered
    #[serde(rename = "medicationCodeableConcept")]
    pub medication_codeable_concept: CodeableConcept,

    #[serde(rename = "medicationReference")]
    pub medication_reference: Box<Reference>,

    /// Who received medication
    pub subject: Box<Reference>,

    /// Encounter or Episode of Care administered as part of
    pub context: Option<Box<Reference>>,

    /// Additional information to support administration
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Box<Reference>>>,

    /// Start and end time of administration
    #[serde(rename = "effectiveDateTime")]
    pub effective_date_time: String,

    #[serde(rename = "effectivePeriod")]
    pub effective_period: Period,

    /// Who performed the medication administration and what they did
    pub performer: Option<Vec<MedicationAdministrationPerformer>>,

    /// Reason administration performed
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Condition or observation that supports why the medication was administered
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// Request administration performed against
    pub request: Option<Box<Reference>>,

    /// Device used to administer
    pub device: Option<Vec<Box<Reference>>>,

    /// Information about the administration
    pub note: Option<Vec<Annotation>>,

    /// Details of how medication was taken
    pub dosage: Option<MedicationAdministrationDosage>,

    /// A list of events of interest in the lifecycle
    #[serde(rename = "eventHistory")]
    pub event_history: Option<Vec<Box<Reference>>>,
}
