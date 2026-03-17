//! FHIR R4B RelatedPerson Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// A language which may be used to communicate with about the patient's health
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedPersonCommunication {
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

/// Information about a person that is involved in the care for a patient, but who is not the target of healthcare, nor has a formal responsibility in the care process.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedPerson {
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

    /// Whether this related person's record is in active use
    pub active: Option<bool>,

    /// The patient this person is related to
    pub patient: Box<Reference>,

    /// The nature of the relationship
    pub relationship: Option<Vec<CodeableConcept>>,

    /// A name associated with the person
    pub name: Option<Vec<HumanName>>,

    /// A contact detail for the person
    pub telecom: Option<Vec<ContactPoint>>,

    /// male | female | other | unknown
    pub gender: Option<String>,

    /// The date on which the related person was born
    #[serde(rename = "birthDate")]
    pub birth_date: Option<String>,

    /// Address where the related person can be contacted or visited
    pub address: Option<Vec<Address>>,

    /// Image of the person
    pub photo: Option<Vec<Attachment>>,

    /// Period of time that this relationship is considered valid
    pub period: Option<Period>,

    /// A language which may be used to communicate with about the patient's health
    pub communication: Option<Vec<RelatedPersonCommunication>>,
}
