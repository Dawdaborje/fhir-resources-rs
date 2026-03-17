//! FHIR R5 Organization Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Qualifications, certifications, accreditations, licenses, training, etc. pertaining to the provision of care
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationQualification {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// An identifier for this qualification for the organization
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Coded representation of the qualification
    pub code: CodeableConcept,

    /// Period during which the qualification is valid
    pub period: Option<Period>,

    /// Organization that regulates and issues the qualification
    pub issuer: Option<Box<Reference>>,
}

/// A formally or informally recognized grouping of people or organizations formed for the purpose of achieving some form of collective action. Includes companies, institutions, corporations, departmen...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
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

    /// Identifies this organization across multiple systems
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Whether the organization's record is still in active use
    pub active: Option<bool>,

    /// Kind of organization
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Name used for the organization
    pub name: Option<String>,

    /// A list of alternate names that the organization is known as, or was known as in the past
    pub alias: Option<Vec<String>>,

    /// Additional details about the Organization that could be displayed as further information to identify the Organization beyond its name
    pub description: Option<String>,

    /// Official contact details for the Organization
    pub contact: Option<Vec<ExtendedContactDetail>>,

    /// The organization of which this organization forms a part
    #[serde(rename = "partOf")]
    pub part_of: Option<Box<Reference>>,

    /// Technical endpoints providing access to services operated for the organization
    pub endpoint: Option<Vec<Box<Reference>>>,

    /// Qualifications, certifications, accreditations, licenses, training, etc. pertaining to the provision of care
    pub qualification: Option<Vec<OrganizationQualification>>,
}
