//! FHIR R4 Appointment Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Participants involved in appointment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Role of participant in the appointment
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Person, Location/HealthcareService or Device
    pub actor: Option<Box<Reference>>,

    /// required | optional | information-only
    pub required: Option<String>,

    /// accepted | declined | tentative | needs-action
    pub status: String,

    /// Participation period of the actor
    pub period: Option<Period>,
}

/// A booking of a healthcare event among patient(s), practitioner(s), related person(s) and/or device(s) for a specific date/time. This may result in one or more Encounter(s).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Appointment {
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

    /// proposed | pending | booked | arrived | fulfilled | cancelled | noshow | entered-in-error | checked-in | waitlist
    pub status: String,

    /// The coded reason for the appointment being cancelled
    #[serde(rename = "cancelationReason")]
    pub cancelation_reason: Option<CodeableConcept>,

    /// A broad categorization of the service that is to be performed during this appointment
    #[serde(rename = "serviceCategory")]
    pub service_category: Option<Vec<CodeableConcept>>,

    /// The specific service that is to be performed during this appointment
    #[serde(rename = "serviceType")]
    pub service_type: Option<Vec<CodeableConcept>>,

    /// The specialty of a practitioner that would be required to perform the service requested in this appointment
    pub specialty: Option<Vec<CodeableConcept>>,

    /// The style of appointment or patient that has been booked in the slot (not service type)
    #[serde(rename = "appointmentType")]
    pub appointment_type: Option<CodeableConcept>,

    /// Coded reason this appointment is scheduled
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Reason the appointment is to take place (resource)
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// Used to make informed decisions if needing to re-prioritize
    pub priority: Option<u32>,

    /// Shown on a subject line in a meeting request, or appointment list
    pub description: Option<String>,

    /// Additional information to support the appointment
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Box<Reference>>>,

    /// When appointment is to take place
    pub start: Option<String>,

    /// When appointment is to conclude
    pub end: Option<String>,

    /// Can be less than start/end (e.g. estimate)
    #[serde(rename = "minutesDuration")]
    pub minutes_duration: Option<i32>,

    /// The slots that this appointment is filling
    pub slot: Option<Vec<Box<Reference>>>,

    /// The date that this appointment was initially created
    pub created: Option<String>,

    /// Additional comments
    pub comment: Option<String>,

    /// Detailed information and instructions for the patient
    #[serde(rename = "patientInstruction")]
    pub patient_instruction: Option<String>,

    /// The service request this appointment is allocated to assess
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Participants involved in appointment
    pub participant: Vec<AppointmentParticipant>,

    /// Potential date/time interval(s) requested to allocate the appointment within
    #[serde(rename = "requestedPeriod")]
    pub requested_period: Option<Vec<Period>>,
}
