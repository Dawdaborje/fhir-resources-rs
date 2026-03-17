//! FHIR R5 Encounter Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

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

    /// The individual, device, or service participating in the encounter
    pub actor: Option<Box<Reference>>,
}

/// The list of medical reasons that are expected to be addressed during the episode of care
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterReason {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// What the reason value should be used for/as
    #[serde(rename = "use")]
    pub r#use: Option<Vec<CodeableConcept>>,

    /// Reason the encounter takes place (core or reference)
    pub value: Option<Vec<CodeableReference>>,
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

    /// The diagnosis relevant to the encounter
    pub condition: Option<Vec<CodeableReference>>,

    /// Role that this diagnosis has within the encounter (e.g. admission, billing, discharge …)
    #[serde(rename = "use")]
    pub r#use: Option<Vec<CodeableConcept>>,
}

/// Details about the admission to a healthcare service
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterAdmission {
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

    /// Indicates that the patient is being re-admitted
    #[serde(rename = "reAdmission")]
    pub re_admission: Option<CodeableConcept>,

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

    /// The physical type of the location (usually the level in the location hierarchy - bed, room, ward, virtual etc.)
    pub form: Option<CodeableConcept>,

    /// Time period during which the patient was present at the location
    pub period: Option<Period>,
}

/// An interaction between healthcare provider(s), and/or patient(s) for the purpose of providing healthcare service(s) or assessing the health status of patient(s).
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

    /// planned | in-progress | on-hold | discharged | completed | cancelled | discontinued | entered-in-error | unknown
    pub status: String,

    /// Classification of patient encounter context - e.g. Inpatient, outpatient
    pub class: Option<Vec<CodeableConcept>>,

    /// Indicates the urgency of the encounter
    pub priority: Option<CodeableConcept>,

    /// Specific type of encounter (e.g. e-mail consultation, surgical day-care, ...)
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Specific type of service
    #[serde(rename = "serviceType")]
    pub service_type: Option<Vec<CodeableReference>>,

    /// The patient or group related to this encounter
    pub subject: Option<Box<Reference>>,

    /// The current status of the subject in relation to the Encounter
    #[serde(rename = "subjectStatus")]
    pub subject_status: Option<CodeableConcept>,

    /// Episode(s) of care that this encounter should be recorded against
    #[serde(rename = "episodeOfCare")]
    pub episode_of_care: Option<Vec<Box<Reference>>>,

    /// The request that initiated this encounter
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// The group(s) that are allocated to participate in this encounter
    #[serde(rename = "careTeam")]
    pub care_team: Option<Vec<Box<Reference>>>,

    /// Another Encounter this encounter is part of
    #[serde(rename = "partOf")]
    pub part_of: Option<Box<Reference>>,

    /// The organization (facility) responsible for this encounter
    #[serde(rename = "serviceProvider")]
    pub service_provider: Option<Box<Reference>>,

    /// List of participants involved in the encounter
    pub participant: Option<Vec<EncounterParticipant>>,

    /// The appointment that scheduled this encounter
    pub appointment: Option<Vec<Box<Reference>>>,

    /// Connection details of a virtual service (e.g. conference call)
    #[serde(rename = "virtualService")]
    pub virtual_service: Option<Vec<VirtualServiceDetail>>,

    /// The actual start and end time of the encounter
    #[serde(rename = "actualPeriod")]
    pub actual_period: Option<Period>,

    /// The planned start date/time (or admission date) of the encounter
    #[serde(rename = "plannedStartDate")]
    pub planned_start_date: Option<String>,

    /// The planned end date/time (or discharge date) of the encounter
    #[serde(rename = "plannedEndDate")]
    pub planned_end_date: Option<String>,

    /// Actual quantity of time the encounter lasted (less time absent)
    pub length: Option<Duration>,

    /// The list of medical reasons that are expected to be addressed during the episode of care
    pub reason: Option<Vec<EncounterReason>>,

    /// The list of diagnosis relevant to this encounter
    pub diagnosis: Option<Vec<EncounterDiagnosis>>,

    /// The set of accounts that may be used for billing for this Encounter
    pub account: Option<Vec<Box<Reference>>>,

    /// Diet preferences reported by the patient
    #[serde(rename = "dietPreference")]
    pub diet_preference: Option<Vec<CodeableConcept>>,

    /// Wheelchair, translator, stretcher, etc
    #[serde(rename = "specialArrangement")]
    pub special_arrangement: Option<Vec<CodeableConcept>>,

    /// Special courtesies (VIP, board member)
    #[serde(rename = "specialCourtesy")]
    pub special_courtesy: Option<Vec<CodeableConcept>>,

    /// Details about the admission to a healthcare service
    pub admission: Option<EncounterAdmission>,

    /// List of locations where the patient has been
    pub location: Option<Vec<EncounterLocation>>,
}
