//! FHIR R4 Organization Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Contact for the organization for a certain purpose
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationContact {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type of contact
    pub purpose: Option<CodeableConcept>,

    /// A name associated with the contact
    pub name: Option<HumanName>,

    /// Contact details (telephone, email, etc.) for a contact
    pub telecom: Option<Vec<ContactPoint>>,

    /// Visiting or postal addresses for the contact
    pub address: Option<Address>,
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

    /// A contact detail for the organization
    pub telecom: Option<Vec<ContactPoint>>,

    /// An address for the organization
    pub address: Option<Vec<Address>>,

    /// The organization of which this organization forms a part
    #[serde(rename = "partOf")]
    pub part_of: Option<Box<Reference>>,

    /// Contact for the organization for a certain purpose
    pub contact: Option<Vec<OrganizationContact>>,

    /// Technical endpoints providing access to services operated for the organization
    pub endpoint: Option<Vec<Box<Reference>>>,
}
