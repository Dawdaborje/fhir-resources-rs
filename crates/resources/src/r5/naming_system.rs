//! FHIR R5 NamingSystem Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Unique identifiers used for system
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamingSystemUniqueId {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// oid | uuid | uri | iri-stem | v2csmnemonic | other
    #[serde(rename = "type")]
    pub r#type: String,

    /// The unique identifier
    pub value: String,

    /// Is this the id that should be used for this type
    pub preferred: Option<bool>,

    /// Notes about identifier usage
    pub comment: Option<String>,

    /// When is identifier valid?
    pub period: Option<Period>,

    /// Whether the identifier is authoritative
    pub authoritative: Option<bool>,
}

/// A curated namespace that issues unique symbols within that namespace for the identification of concepts, people, devices, etc. Represents a "System" used within the Identifier and Coding data types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamingSystem {
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

    /// Canonical identifier for this naming system, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the naming system (business identifier)
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the naming system
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<String>,

    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,

    /// Name for this naming system (computer friendly)
    pub name: String,

    /// Title for this naming system (human friendly)
    pub title: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// codesystem | identifier | root
    pub kind: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: String,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Who maintains system namespace?
    pub responsible: Option<String>,

    /// e.g. driver, provider, patient, bank etc
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Natural language description of the naming system
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for naming system (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this naming system is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// When the NamingSystem was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the NamingSystem was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the NamingSystem is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// E.g. Education, Treatment, Assessment, etc
    pub topic: Option<Vec<CodeableConcept>>,

    /// Who authored the CodeSystem
    pub author: Option<Vec<ContactDetail>>,

    /// Who edited the NamingSystem
    pub editor: Option<Vec<ContactDetail>>,

    /// Who reviewed the NamingSystem
    pub reviewer: Option<Vec<ContactDetail>>,

    /// Who endorsed the NamingSystem
    pub endorser: Option<Vec<ContactDetail>>,

    /// Additional documentation, citations, etc
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// How/where is it used
    pub usage: Option<String>,

    /// Unique identifiers used for system
    #[serde(rename = "uniqueId")]
    pub unique_id: Vec<NamingSystemUniqueId>,
}
