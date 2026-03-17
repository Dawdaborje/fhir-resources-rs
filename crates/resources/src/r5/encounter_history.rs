//! FHIR R5 EncounterHistory Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Location of the patient at this point in the encounter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterHistoryLocation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Location the encounter takes place
    pub location: Box<Reference>,

    /// The physical type of the location (usually the level in the location hierarchy - bed, room, ward, virtual etc.)
    pub form: Option<CodeableConcept>,
}

/// A record of significant events/milestones key data throughout the history of an Encounter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterHistory {
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

    /// The Encounter associated with this set of historic values
    pub encounter: Option<Box<Reference>>,

    /// Identifier(s) by which this encounter is known
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// planned | in-progress | on-hold | discharged | completed | cancelled | discontinued | entered-in-error | unknown
    pub status: String,

    /// Classification of patient encounter
    pub class: CodeableConcept,

    /// Specific type of encounter
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

    /// The actual start and end time associated with this set of values associated with the encounter
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

    /// Location of the patient at this point in the encounter
    pub location: Option<Vec<EncounterHistoryLocation>>,
}
