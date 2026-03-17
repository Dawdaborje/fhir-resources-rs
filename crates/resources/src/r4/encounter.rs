//! FHIR R4 Encounter Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// List of past encounter statuses
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterStatusHistory {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// planned | arrived | triaged | in-progress | onleave | finished | cancelled +
    pub status: String,

    /// The time that the episode was in the specified status
    pub period: Period,
}

/// List of past encounter classes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterClassHistory {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// inpatient | outpatient | ambulatory | emergency +
    pub class: Coding,

    /// The time that the episode was in the specified class
    pub period: Period,
}

/// List of participants involved in the encounter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Role of participant in encounter
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Period of time during the encounter that the participant participated
    pub period: Option<Period>,

    /// Persons involved in the encounter other than the patient
    pub individual: Option<Box<Reference>>,
}

/// The list of diagnosis relevant to this encounter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The diagnosis or procedure relevant to the encounter
    pub condition: Box<Reference>,

    /// Role that this diagnosis has within the encounter (e.g. admission, billing, discharge …)
    #[serde(rename = "use")]
    pub r#use: Option<CodeableConcept>,

    /// Ranking of the diagnosis (for each role type)
    pub rank: Option<i32>,
}

/// Details about the admission to a healthcare service
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterHospitalization {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Pre-admission identifier
    #[serde(rename = "preAdmissionIdentifier")]
    pub pre_admission_identifier: Option<Box<Identifier>>,

    /// The location/organization from which the patient came before admission
    pub origin: Option<Box<Reference>>,

    /// From where patient was admitted (physician referral, transfer)
    #[serde(rename = "admitSource")]
    pub admit_source: Option<CodeableConcept>,

    /// The type of hospital re-admission that has occurred (if any). If the value is absent, then this is not identified as a readmission
    #[serde(rename = "reAdmission")]
    pub re_admission: Option<CodeableConcept>,

    /// Diet preferences reported by the patient
    #[serde(rename = "dietPreference")]
    pub diet_preference: Option<Vec<CodeableConcept>>,

    /// Special courtesies (VIP, board member)
    #[serde(rename = "specialCourtesy")]
    pub special_courtesy: Option<Vec<CodeableConcept>>,

    /// Wheelchair, translator, stretcher, etc.
    #[serde(rename = "specialArrangement")]
    pub special_arrangement: Option<Vec<CodeableConcept>>,

    /// Location/organization to which the patient is discharged
    pub destination: Option<Box<Reference>>,

    /// Category or kind of location after discharge
    #[serde(rename = "dischargeDisposition")]
    pub discharge_disposition: Option<CodeableConcept>,
}

/// List of locations where the patient has been
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterLocation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Location the encounter takes place
    pub location: Box<Reference>,

    /// planned | active | reserved | completed
    pub status: Option<String>,

    /// The physical type of the location (usually the level in the location hierachy - bed room ward etc.)
    #[serde(rename = "physicalType")]
    pub physical_type: Option<CodeableConcept>,

    /// Time period during which the patient was present at the location
    pub period: Option<Period>,
}

/// An interaction between a patient and healthcare provider(s) for the purpose of providing healthcare service(s) or assessing the health status of a patient.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
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

    /// Identifier(s) by which this encounter is known
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// planned | arrived | triaged | in-progress | onleave | finished | cancelled +
    pub status: String,

    /// List of past encounter statuses
    #[serde(rename = "statusHistory")]
    pub status_history: Option<Vec<EncounterStatusHistory>>,

    /// Classification of patient encounter
    pub class: Coding,

    /// List of past encounter classes
    #[serde(rename = "classHistory")]
    pub class_history: Option<Vec<EncounterClassHistory>>,

    /// Specific type of encounter
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Specific type of service
    #[serde(rename = "serviceType")]
    pub service_type: Option<CodeableConcept>,

    /// Indicates the urgency of the encounter
    pub priority: Option<CodeableConcept>,

    /// The patient or group present at the encounter
    pub subject: Option<Box<Reference>>,

    /// Episode(s) of care that this encounter should be recorded against
    #[serde(rename = "episodeOfCare")]
    pub episode_of_care: Option<Vec<Box<Reference>>>,

    /// The ServiceRequest that initiated this encounter
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// List of participants involved in the encounter
    pub participant: Option<Vec<EncounterParticipant>>,

    /// The appointment that scheduled this encounter
    pub appointment: Option<Vec<Box<Reference>>>,

    /// The start and end time of the encounter
    pub period: Option<Period>,

    /// Quantity of time the encounter lasted (less time absent)
    pub length: Option<Duration>,

    /// Coded reason the encounter takes place
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Reason the encounter takes place (reference)
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// The list of diagnosis relevant to this encounter
    pub diagnosis: Option<Vec<EncounterDiagnosis>>,

    /// The set of accounts that may be used for billing for this Encounter
    pub account: Option<Vec<Box<Reference>>>,

    /// Details about the admission to a healthcare service
    pub hospitalization: Option<EncounterHospitalization>,

    /// List of locations where the patient has been
    pub location: Option<Vec<EncounterLocation>>,

    /// The organization (facility) responsible for this encounter
    #[serde(rename = "serviceProvider")]
    pub service_provider: Option<Box<Reference>>,

    /// Another Encounter this encounter is part of
    #[serde(rename = "partOf")]
    pub part_of: Option<Box<Reference>>,
}
