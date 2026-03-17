//! FHIR R5 Appointment Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
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

    /// Participation period of the actor
    pub period: Option<Period>,

    /// The individual, device, location, or service participating in the appointment
    pub actor: Option<Box<Reference>>,

    /// The participant is required to attend (optional when false)
    pub required: Option<bool>,

    /// accepted | declined | tentative | needs-action
    pub status: String,
}

/// Details of the recurrence pattern/template used to generate occurrences
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentRecurrenceTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The timezone of the occurrences
    pub timezone: Option<CodeableConcept>,

    /// The frequency of the recurrence
    #[serde(rename = "recurrenceType")]
    pub recurrence_type: CodeableConcept,

    /// The date when the recurrence should end
    #[serde(rename = "lastOccurrenceDate")]
    pub last_occurrence_date: Option<String>,

    /// The number of planned occurrences
    #[serde(rename = "occurrenceCount")]
    pub occurrence_count: Option<i32>,

    /// Specific dates for a recurring set of appointments (no template)
    #[serde(rename = "occurrenceDate")]
    pub occurrence_date: Option<Vec<String>>,

    /// Information about weekly recurring appointments
    #[serde(rename = "weeklyTemplate")]
    pub weekly_template: Option<AppointmentRecurrenceTemplateWeeklyTemplate>,

    /// Information about monthly recurring appointments
    #[serde(rename = "monthlyTemplate")]
    pub monthly_template: Option<AppointmentRecurrenceTemplateMonthlyTemplate>,

    /// Information about yearly recurring appointments
    #[serde(rename = "yearlyTemplate")]
    pub yearly_template: Option<AppointmentRecurrenceTemplateYearlyTemplate>,

    /// Any dates that should be excluded from the series
    #[serde(rename = "excludingDate")]
    pub excluding_date: Option<Vec<String>>,

    /// Any recurrence IDs that should be excluded from the recurrence
    #[serde(rename = "excludingRecurrenceId")]
    pub excluding_recurrence_id: Option<Vec<i32>>,
}

/// Information about weekly recurring appointments
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentRecurrenceTemplateWeeklyTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Recurs on Mondays
    pub monday: Option<bool>,

    /// Recurs on Tuesday
    pub tuesday: Option<bool>,

    /// Recurs on Wednesday
    pub wednesday: Option<bool>,

    /// Recurs on Thursday
    pub thursday: Option<bool>,

    /// Recurs on Friday
    pub friday: Option<bool>,

    /// Recurs on Saturday
    pub saturday: Option<bool>,

    /// Recurs on Sunday
    pub sunday: Option<bool>,

    /// Recurs every nth week
    #[serde(rename = "weekInterval")]
    pub week_interval: Option<i32>,
}

/// Information about monthly recurring appointments
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentRecurrenceTemplateMonthlyTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Recurs on a specific day of the month
    #[serde(rename = "dayOfMonth")]
    pub day_of_month: Option<i32>,

    /// Indicates which week of the month the appointment should occur
    #[serde(rename = "nthWeekOfMonth")]
    pub nth_week_of_month: Option<Coding>,

    /// Indicates which day of the week the appointment should occur
    #[serde(rename = "dayOfWeek")]
    pub day_of_week: Option<Coding>,

    /// Recurs every nth month
    #[serde(rename = "monthInterval")]
    pub month_interval: i32,
}

/// Information about yearly recurring appointments
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentRecurrenceTemplateYearlyTemplate {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Recurs every nth year
    #[serde(rename = "yearInterval")]
    pub year_interval: i32,
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
    #[serde(rename = "cancellationReason")]
    pub cancellation_reason: Option<CodeableConcept>,

    /// Classification when becoming an encounter
    pub class: Option<Vec<CodeableConcept>>,

    /// A broad categorization of the service that is to be performed during this appointment
    #[serde(rename = "serviceCategory")]
    pub service_category: Option<Vec<CodeableConcept>>,

    /// The specific service that is to be performed during this appointment
    #[serde(rename = "serviceType")]
    pub service_type: Option<Vec<CodeableReference>>,

    /// The specialty of a practitioner that would be required to perform the service requested in this appointment
    pub specialty: Option<Vec<CodeableConcept>>,

    /// The style of appointment or patient that has been booked in the slot (not service type)
    #[serde(rename = "appointmentType")]
    pub appointment_type: Option<CodeableConcept>,

    /// Reason this appointment is scheduled
    pub reason: Option<Vec<CodeableReference>>,

    /// Used to make informed decisions if needing to re-prioritize
    pub priority: Option<CodeableConcept>,

    /// Shown on a subject line in a meeting request, or appointment list
    pub description: Option<String>,

    /// Appointment replaced by this Appointment
    pub replaces: Option<Vec<Box<Reference>>>,

    /// Connection details of a virtual service (e.g. conference call)
    #[serde(rename = "virtualService")]
    pub virtual_service: Option<Vec<VirtualServiceDetail>>,

    /// Additional information to support the appointment
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Box<Reference>>>,

    /// The previous appointment in a series
    #[serde(rename = "previousAppointment")]
    pub previous_appointment: Option<Box<Reference>>,

    /// The originating appointment in a recurring set of appointments
    #[serde(rename = "originatingAppointment")]
    pub originating_appointment: Option<Box<Reference>>,

    /// When appointment is to take place
    pub start: Option<String>,

    /// When appointment is to conclude
    pub end: Option<String>,

    /// Can be less than start/end (e.g. estimate)
    #[serde(rename = "minutesDuration")]
    pub minutes_duration: Option<i32>,

    /// Potential date/time interval(s) requested to allocate the appointment within
    #[serde(rename = "requestedPeriod")]
    pub requested_period: Option<Vec<Period>>,

    /// The slots that this appointment is filling
    pub slot: Option<Vec<Box<Reference>>>,

    /// The set of accounts that may be used for billing for this Appointment
    pub account: Option<Vec<Box<Reference>>>,

    /// The date that this appointment was initially created
    pub created: Option<String>,

    /// When the appointment was cancelled
    #[serde(rename = "cancellationDate")]
    pub cancellation_date: Option<String>,

    /// Additional comments
    pub note: Option<Vec<Annotation>>,

    /// Detailed information and instructions for the patient
    #[serde(rename = "patientInstruction")]
    pub patient_instruction: Option<Vec<CodeableReference>>,

    /// The request this appointment is allocated to assess
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// The patient or group associated with the appointment
    pub subject: Option<Box<Reference>>,

    /// Participants involved in appointment
    pub participant: Vec<AppointmentParticipant>,

    /// The sequence number in the recurrence
    #[serde(rename = "recurrenceId")]
    pub recurrence_id: Option<i32>,

    /// Indicates that this appointment varies from a recurrence pattern
    #[serde(rename = "occurrenceChanged")]
    pub occurrence_changed: Option<bool>,

    /// Details of the recurrence pattern/template used to generate occurrences
    #[serde(rename = "recurrenceTemplate")]
    pub recurrence_template: Option<Vec<AppointmentRecurrenceTemplate>>,
}
