//! FHIR R5 MetadataResource Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Common Interface declaration for conformance and knowledge artifact resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataResource {
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

    /// Canonical identifier for this {{title}}, represented as an absolute URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the {{title}}
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the {{title}}
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithm")]
    pub version_algorithm: Option<serde_json::Value>,

    /// Name for this {{title}} (computer friendly)
    pub name: Option<String>,

    /// Name for this {{title}} (human friendly)
    pub title: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the {{title}}
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for {{title}} (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this {{title}} is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// When the {{title}} was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the {{title}} was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the {{title}} is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// E.g. Education, Treatment, Assessment, etc
    pub topic: Option<Vec<CodeableConcept>>,

    /// Who authored the {{title}}
    pub author: Option<Vec<ContactDetail>>,

    /// Who edited the {{title}}
    pub editor: Option<Vec<ContactDetail>>,

    /// Who reviewed the {{title}}
    pub reviewer: Option<Vec<ContactDetail>>,

    /// Who endorsed the {{title}}
    pub endorser: Option<Vec<ContactDetail>>,

    /// Additional documentation, citations, etc
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
}
