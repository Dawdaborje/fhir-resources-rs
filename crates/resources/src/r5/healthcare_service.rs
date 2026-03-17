//! FHIR R5 HealthcareService Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
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

/// The details of a healthcare service available at a location or in a catalog. In the case where there is a hierarchy of services (for example, Lab -> Pathology -> Wound Cultures), this can be repres...
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

    /// The service within which this service is offered
    #[serde(rename = "offeredIn")]
    pub offered_in: Option<Vec<Box<Reference>>>,

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

    /// Official contact details for the HealthcareService
    pub contact: Option<Vec<ExtendedContactDetail>>,

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

    /// Times the healthcare service is available (including exceptions)
    pub availability: Option<Vec<Availability>>,

    /// Technical endpoints providing access to electronic services operated for the healthcare service
    pub endpoint: Option<Vec<Box<Reference>>>,
}
