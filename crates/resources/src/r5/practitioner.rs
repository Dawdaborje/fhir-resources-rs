//! FHIR R5 Practitioner Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Qualifications, certifications, accreditations, licenses, training, etc. pertaining to the provision of care
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PractitionerQualification {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// An identifier for this qualification for the practitioner
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Coded representation of the qualification
    pub code: CodeableConcept,

    /// Period during which the qualification is valid
    pub period: Option<Period>,

    /// Organization that regulates and issues the qualification
    pub issuer: Option<Box<Reference>>,
}

/// A language which may be used to communicate with the practitioner
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PractitionerCommunication {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The language code used to communicate with the practitioner
    pub language: CodeableConcept,

    /// Language preference indicator
    pub preferred: Option<bool>,
}

/// A person who is directly or indirectly involved in the provisioning of healthcare or related services.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Practitioner {
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

    /// An identifier for the person as this agent
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Whether this practitioner's record is in active use
    pub active: Option<bool>,

    /// The name(s) associated with the practitioner
    pub name: Option<Vec<HumanName>>,

    /// A contact detail for the practitioner (that apply to all roles)
    pub telecom: Option<Vec<ContactPoint>>,

    /// male | female | other | unknown
    pub gender: Option<String>,

    /// The date on which the practitioner was born
    #[serde(rename = "birthDate")]
    pub birth_date: Option<String>,

    /// Indicates if the practitioner is deceased or not
    #[serde(rename = "deceasedBoolean")]
    pub deceased_boolean: Option<bool>,

    #[serde(rename = "deceasedDateTime")]
    pub deceased_date_time: Option<String>,

    /// Address(es) of the practitioner that are not role specific (typically home address)
    pub address: Option<Vec<Address>>,

    /// Image of the person
    pub photo: Option<Vec<Attachment>>,

    /// Qualifications, certifications, accreditations, licenses, training, etc. pertaining to the provision of care
    pub qualification: Option<Vec<PractitionerQualification>>,

    /// A language which may be used to communicate with the practitioner
    pub communication: Option<Vec<PractitionerCommunication>>,
}
