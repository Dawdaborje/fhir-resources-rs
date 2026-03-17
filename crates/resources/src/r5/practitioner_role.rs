//! FHIR R5 PractitionerRole Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// A specific set of Roles/Locations/specialties/services that a practitioner may perform, or has performed at an organization during a period of time.
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

    /// Identifiers for a role/location
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Whether this practitioner role record is in active use
    pub active: Option<bool>,

    /// The period during which the practitioner is authorized to perform in these role(s)
    pub period: Option<Period>,

    /// Practitioner that provides services for the organization
    pub practitioner: Option<Box<Reference>>,

    /// Organization where the roles are available
    pub organization: Option<Box<Reference>>,

    /// Roles which this practitioner may perform
    pub code: Option<Vec<CodeableConcept>>,

    /// Specific specialty of the practitioner
    pub specialty: Option<Vec<CodeableConcept>>,

    /// Location(s) where the practitioner provides care
    pub location: Option<Vec<Box<Reference>>>,

    /// Healthcare services provided for this role's Organization/Location(s)
    #[serde(rename = "healthcareService")]
    pub healthcare_service: Option<Vec<Box<Reference>>>,

    /// Official contact details relating to this PractitionerRole
    pub contact: Option<Vec<ExtendedContactDetail>>,

    /// Collection of characteristics (attributes)
    pub characteristic: Option<Vec<CodeableConcept>>,

    /// A language the practitioner (in this role) can use in patient communication
    pub communication: Option<Vec<CodeableConcept>>,

    /// Times the Practitioner is available at this location and/or healthcare service (including exceptions)
    pub availability: Option<Vec<Availability>>,

    /// Endpoints for interacting with the practitioner in this role
    pub endpoint: Option<Vec<Box<Reference>>>,
}
