//! FHIR R5 AppointmentResponse Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// A reply to an appointment request for a patient and/or practitioner(s), such as a confirmation or rejection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentResponse {
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

    /// External Ids for this item
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Appointment this response relates to
    pub appointment: Box<Reference>,

    /// Indicator for a counter proposal
    #[serde(rename = "proposedNewTime")]
    pub proposed_new_time: Option<bool>,

    /// Time from appointment, or requested new start time
    pub start: Option<String>,

    /// Time from appointment, or requested new end time
    pub end: Option<String>,

    /// Role of participant in the appointment
    #[serde(rename = "participantType")]
    pub participant_type: Option<Vec<CodeableConcept>>,

    /// Person(s), Location, HealthcareService, or Device
    pub actor: Option<Box<Reference>>,

    /// accepted | declined | tentative | needs-action | entered-in-error
    #[serde(rename = "participantStatus")]
    pub participant_status: String,

    /// Additional comments
    pub comment: Option<String>,

    /// This response is for all occurrences in a recurring request
    pub recurring: Option<bool>,

    /// Original date within a recurring request
    #[serde(rename = "occurrenceDate")]
    pub occurrence_date: Option<String>,

    /// The recurrence ID of the specific recurring request
    #[serde(rename = "recurrenceId")]
    pub recurrence_id: Option<i32>,
}
