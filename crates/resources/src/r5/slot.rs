//! FHIR R5 Slot Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// A slot of time on a schedule that may be available for booking appointments.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
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

    /// A broad categorization of the service that is to be performed during this appointment
    #[serde(rename = "serviceCategory")]
    pub service_category: Option<Vec<CodeableConcept>>,

    /// The type of appointments that can be booked into this slot (ideally this would be an identifiable service - which is at a location, rather than the location itself). If provided then this overrides...
    #[serde(rename = "serviceType")]
    pub service_type: Option<Vec<CodeableReference>>,

    /// The specialty of a practitioner that would be required to perform the service requested in this appointment
    pub specialty: Option<Vec<CodeableConcept>>,

    /// The style of appointment or patient that may be booked in the slot (not service type)
    #[serde(rename = "appointmentType")]
    pub appointment_type: Option<Vec<CodeableConcept>>,

    /// The schedule resource that this slot defines an interval of status information
    pub schedule: Box<Reference>,

    /// busy | free | busy-unavailable | busy-tentative | entered-in-error
    pub status: String,

    /// Date/Time that the slot is to begin
    pub start: String,

    /// Date/Time that the slot is to conclude
    pub end: String,

    /// This slot has already been overbooked, appointments are unlikely to be accepted for this time
    pub overbooked: Option<bool>,

    /// Comments on the slot to describe any extended information. Such as custom constraints on the slot
    pub comment: Option<String>,
}
