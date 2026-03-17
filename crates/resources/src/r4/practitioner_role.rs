//! FHIR R4 PractitionerRole Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Times the Service Site is available
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PractitionerRoleAvailableTime {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// mon | tue | wed | thu | fri | sat | sun
    #[serde(rename = "daysOfWeek")]
    pub days_of_week: Option<Vec<String>>,

    /// Always available? e.g. 24 hour service
    #[serde(rename = "allDay")]
    pub all_day: Option<bool>,

    /// Opening time of day (ignored if allDay = true)
    #[serde(rename = "availableStartTime")]
    pub available_start_time: Option<String>,

    /// Closing time of day (ignored if allDay = true)
    #[serde(rename = "availableEndTime")]
    pub available_end_time: Option<String>,
}

/// Not available during this time due to provided reason
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PractitionerRoleNotAvailable {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reason presented to the user explaining why time not available
    pub description: String,

    /// Service not available from this date
    pub during: Option<Period>,
}

/// A specific set of Roles/Locations/specialties/services that a practitioner may perform at an organization for a period of time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PractitionerRole {
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

    /// Business Identifiers that are specific to a role/location
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Whether this practitioner role record is in active use
    pub active: Option<bool>,

    /// The period during which the practitioner is authorized to perform in these role(s)
    pub period: Option<Period>,

    /// Practitioner that is able to provide the defined services for the organization
    pub practitioner: Option<Box<Reference>>,

    /// Organization where the roles are available
    pub organization: Option<Box<Reference>>,

    /// Roles which this practitioner may perform
    pub code: Option<Vec<CodeableConcept>>,

    /// Specific specialty of the practitioner
    pub specialty: Option<Vec<CodeableConcept>>,

    /// The location(s) at which this practitioner provides care
    pub location: Option<Vec<Box<Reference>>>,

    /// The list of healthcare services that this worker provides for this role's Organization/Location(s)
    #[serde(rename = "healthcareService")]
    pub healthcare_service: Option<Vec<Box<Reference>>>,

    /// Contact details that are specific to the role/location/service
    pub telecom: Option<Vec<ContactPoint>>,

    /// Times the Service Site is available
    #[serde(rename = "availableTime")]
    pub available_time: Option<Vec<PractitionerRoleAvailableTime>>,

    /// Not available during this time due to provided reason
    #[serde(rename = "notAvailable")]
    pub not_available: Option<Vec<PractitionerRoleNotAvailable>>,

    /// Description of availability exceptions
    #[serde(rename = "availabilityExceptions")]
    pub availability_exceptions: Option<String>,

    /// Technical endpoints providing access to services operated for the practitioner with this role
    pub endpoint: Option<Vec<Box<Reference>>>,
}
