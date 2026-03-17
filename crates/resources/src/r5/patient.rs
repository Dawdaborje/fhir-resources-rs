//! FHIR R5 Patient Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// A contact party (e.g. guardian, partner, friend) for the patient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatientContact {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The kind of relationship
    pub relationship: Option<Vec<CodeableConcept>>,

    /// A name associated with the contact person
    pub name: Option<HumanName>,

    /// A contact detail for the person
    pub telecom: Option<Vec<ContactPoint>>,

    /// Address for the contact person
    pub address: Option<Address>,

    /// male | female | other | unknown
    pub gender: Option<String>,

    /// Organization that is associated with the contact
    pub organization: Option<Box<Reference>>,

    /// The period during which this contact person or organization is valid to be contacted relating to this patient
    pub period: Option<Period>,
}

/// A language which may be used to communicate with the patient about his or her health
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatientCommunication {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The language which can be used to communicate with the patient about his or her health
    pub language: CodeableConcept,

    /// Language preference indicator
    pub preferred: Option<bool>,
}

/// Link to a Patient or RelatedPerson resource that concerns the same actual individual
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatientLink {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The other patient or related person resource that the link refers to
    pub other: Box<Reference>,

    /// replaced-by | replaces | refer | seealso
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Demographics and other administrative information about an individual or animal receiving care or other health-related services.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
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

    /// An identifier for this patient
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Whether this patient's record is in active use
    pub active: Option<bool>,

    /// A name associated with the patient
    pub name: Option<Vec<HumanName>>,

    /// A contact detail for the individual
    pub telecom: Option<Vec<ContactPoint>>,

    /// male | female | other | unknown
    pub gender: Option<String>,

    /// The date of birth for the individual
    #[serde(rename = "birthDate")]
    pub birth_date: Option<String>,

    /// Indicates if the individual is deceased or not
    pub deceased: Option<serde_json::Value>,

    /// An address for the individual
    pub address: Option<Vec<Address>>,

    /// Marital (civil) status of a patient
    #[serde(rename = "maritalStatus")]
    pub marital_status: Option<CodeableConcept>,

    /// Whether patient is part of a multiple birth
    #[serde(rename = "multipleBirth")]
    pub multiple_birth: Option<serde_json::Value>,

    /// Image of the patient
    pub photo: Option<Vec<Attachment>>,

    /// A contact party (e.g. guardian, partner, friend) for the patient
    pub contact: Option<Vec<PatientContact>>,

    /// A language which may be used to communicate with the patient about his or her health
    pub communication: Option<Vec<PatientCommunication>>,

    /// Patient's nominated primary care provider
    #[serde(rename = "generalPractitioner")]
    pub general_practitioner: Option<Vec<Box<Reference>>>,

    /// Organization that is the custodian of the patient record
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Box<Reference>>,

    /// Link to a Patient or RelatedPerson resource that concerns the same actual individual
    pub link: Option<Vec<PatientLink>>,
}
