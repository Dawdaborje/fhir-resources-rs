//! FHIR R4 OrganizationAffiliation Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Defines an affiliation/assotiation/relationship between 2 distinct oganizations, that is not a part-of relationship/sub-division relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationAffiliation {
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

    /// Business identifiers that are specific to this role
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Whether this organization affiliation record is in active use
    pub active: Option<bool>,

    /// The period during which the participatingOrganization is affiliated with the primary organization
    pub period: Option<Period>,

    /// Organization where the role is available
    pub organization: Option<Box<Reference>>,

    /// Organization that provides/performs the role (e.g. providing services or is a member of)
    #[serde(rename = "participatingOrganization")]
    pub participating_organization: Option<Box<Reference>>,

    /// Health insurance provider network in which the participatingOrganization provides the role's services (if defined) at the indicated locations (if defined)
    pub network: Option<Vec<Box<Reference>>>,

    /// Definition of the role the participatingOrganization plays
    pub code: Option<Vec<CodeableConcept>>,

    /// Specific specialty of the participatingOrganization in the context of the role
    pub specialty: Option<Vec<CodeableConcept>>,

    /// The location(s) at which the role occurs
    pub location: Option<Vec<Box<Reference>>>,

    /// Healthcare services provided through the role
    #[serde(rename = "healthcareService")]
    pub healthcare_service: Option<Vec<Box<Reference>>>,

    /// Contact details at the participatingOrganization relevant to this Affiliation
    pub telecom: Option<Vec<ContactPoint>>,

    /// Technical endpoints providing access to services operated for this role
    pub endpoint: Option<Vec<Box<Reference>>>,
}
