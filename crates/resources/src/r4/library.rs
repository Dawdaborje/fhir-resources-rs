//! FHIR R4 Library Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// The Library resource is a general-purpose container for knowledge asset definitions. It can be used to describe and expose existing knowledge assets such as logic libraries and information model de...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
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

    /// Canonical identifier for this library, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the library
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the library
    pub version: Option<String>,

    /// Name for this library (computer friendly)
    pub name: Option<String>,

    /// Name for this library (human friendly)
    pub title: Option<String>,

    /// Subordinate title of the library
    pub subtitle: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// logic-library | model-definition | asset-collection | module-definition
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Type of individual the library content is focused on
    pub subject: Option<serde_json::Value>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the library
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for library (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this library is defined
    pub purpose: Option<String>,

    /// Describes the clinical usage of the library
    pub usage: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// When the library was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the library was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the library is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// E.g. Education, Treatment, Assessment, etc.
    pub topic: Option<Vec<CodeableConcept>>,

    /// Who authored the content
    pub author: Option<Vec<ContactDetail>>,

    /// Who edited the content
    pub editor: Option<Vec<ContactDetail>>,

    /// Who reviewed the content
    pub reviewer: Option<Vec<ContactDetail>>,

    /// Who endorsed the content
    pub endorser: Option<Vec<ContactDetail>>,

    /// Additional documentation, citations, etc.
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// Parameters defined by the library
    pub parameter: Option<Vec<ParameterDefinition>>,

    /// What data is referenced by this library
    #[serde(rename = "dataRequirement")]
    pub data_requirement: Option<Vec<DataRequirement>>,

    /// Contents of the library, either embedded or referenced
    pub content: Option<Vec<Attachment>>,
}
