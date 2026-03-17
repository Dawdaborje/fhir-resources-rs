//! FHIR R5 Person Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// A language which may be used to communicate with the person about his or her health
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonCommunication {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The language which can be used to communicate with the person about his or her health
    pub language: CodeableConcept,

    /// Language preference indicator
    pub preferred: Option<bool>,
}

/// Link to a resource that concerns the same actual person
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonLink {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The resource to which this actual person is associated
    pub target: Box<Reference>,

    /// level1 | level2 | level3 | level4
    pub assurance: Option<String>,
}

/// Demographics and administrative information about a person independent of a specific health-related context.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
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

    /// A human identifier for this person
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// This person's record is in active use
    pub active: Option<bool>,

    /// A name associated with the person
    pub name: Option<Vec<HumanName>>,

    /// A contact detail for the person
    pub telecom: Option<Vec<ContactPoint>>,

    /// male | female | other | unknown
    pub gender: Option<String>,

    /// The date on which the person was born
    #[serde(rename = "birthDate")]
    pub birth_date: Option<String>,

    /// Indicates if the individual is deceased or not
    #[serde(rename = "deceasedBoolean")]
    pub deceased_boolean: Option<bool>,

    #[serde(rename = "deceasedDateTime")]
    pub deceased_date_time: Option<String>,

    /// One or more addresses for the person
    pub address: Option<Vec<Address>>,

    /// Marital (civil) status of a person
    #[serde(rename = "maritalStatus")]
    pub marital_status: Option<CodeableConcept>,

    /// Image of the person
    pub photo: Option<Vec<Attachment>>,

    /// A language which may be used to communicate with the person about his or her health
    pub communication: Option<Vec<PersonCommunication>>,

    /// The organization that is the custodian of the person record
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Box<Reference>>,

    /// Link to a resource that concerns the same actual person
    pub link: Option<Vec<PersonLink>>,
}
