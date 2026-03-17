//! FHIR R4B HealthcareService Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Specific eligibility requirements required to use the service
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthcareServiceEligibility {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Coded value for the eligibility
    pub code: Option<CodeableConcept>,

    /// Describes the eligibility conditions for the service
    pub comment: Option<String>,
}

/// Times the Service Site is available
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthcareServiceAvailableTime {
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
pub struct HealthcareServiceNotAvailable {
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

/// The details of a healthcare service available at a location.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthcareService {
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

    /// External identifiers for this item
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Whether this HealthcareService record is in active use
    pub active: Option<bool>,

    /// Organization that provides this service
    #[serde(rename = "providedBy")]
    pub provided_by: Option<Box<Reference>>,

    /// Broad category of service being performed or delivered
    pub category: Option<Vec<CodeableConcept>>,

    /// Type of service that may be delivered or performed
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Specialties handled by the HealthcareService
    pub specialty: Option<Vec<CodeableConcept>>,

    /// Location(s) where service may be provided
    pub location: Option<Vec<Box<Reference>>>,

    /// Description of service as presented to a consumer while searching
    pub name: Option<String>,

    /// Additional description and/or any specific issues not covered elsewhere
    pub comment: Option<String>,

    /// Extra details about the service that can't be placed in the other fields
    #[serde(rename = "extraDetails")]
    pub extra_details: Option<String>,

    /// Facilitates quick identification of the service
    pub photo: Option<Attachment>,

    /// Contacts related to the healthcare service
    pub telecom: Option<Vec<ContactPoint>>,

    /// Location(s) service is intended for/available to
    #[serde(rename = "coverageArea")]
    pub coverage_area: Option<Vec<Box<Reference>>>,

    /// Conditions under which service is available/offered
    #[serde(rename = "serviceProvisionCode")]
    pub service_provision_code: Option<Vec<CodeableConcept>>,

    /// Specific eligibility requirements required to use the service
    pub eligibility: Option<Vec<HealthcareServiceEligibility>>,

    /// Programs that this service is applicable to
    pub program: Option<Vec<CodeableConcept>>,

    /// Collection of characteristics (attributes)
    pub characteristic: Option<Vec<CodeableConcept>>,

    /// The language that this service is offered in
    pub communication: Option<Vec<CodeableConcept>>,

    /// Ways that the service accepts referrals
    #[serde(rename = "referralMethod")]
    pub referral_method: Option<Vec<CodeableConcept>>,

    /// If an appointment is required for access to this service
    #[serde(rename = "appointmentRequired")]
    pub appointment_required: Option<bool>,

    /// Times the Service Site is available
    #[serde(rename = "availableTime")]
    pub available_time: Option<Vec<HealthcareServiceAvailableTime>>,

    /// Not available during this time due to provided reason
    #[serde(rename = "notAvailable")]
    pub not_available: Option<Vec<HealthcareServiceNotAvailable>>,

    /// Description of availability exceptions
    #[serde(rename = "availabilityExceptions")]
    pub availability_exceptions: Option<String>,

    /// Technical endpoints providing access to electronic services operated for the healthcare service
    pub endpoint: Option<Vec<Box<Reference>>>,
}
