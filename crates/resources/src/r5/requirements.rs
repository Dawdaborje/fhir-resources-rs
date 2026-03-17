//! FHIR R5 Requirements Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Actual statement as markdown
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequirementsStatement {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Key that identifies this statement
    pub key: String,

    /// Short Human label for this statement
    pub label: Option<String>,

    /// SHALL | SHOULD | MAY | SHOULD-NOT
    pub conformance: Option<Vec<String>>,

    /// Set to true if requirements statement is conditional
    pub conditionality: Option<bool>,

    /// The actual requirement
    pub requirement: String,

    /// Another statement this clarifies/restricts ([url#]key)
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<String>,

    /// A larger requirement that this requirement helps to refine and enable
    pub parent: Option<String>,

    /// Design artifact that satisfies this requirement
    #[serde(rename = "satisfiedBy")]
    pub satisfied_by: Option<Vec<String>>,

    /// External artifact (rule/document etc. that) created this requirement
    pub reference: Option<Vec<String>>,

    /// Who asked for this statement
    pub source: Option<Vec<Box<Reference>>>,
}

/// The Requirements resource is used to describe an actor - a human or an application that plays a role in data exchange, and that may have obligations associated with the role the actor plays.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requirements {
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

    /// Canonical identifier for this Requirements, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the Requirements (business identifier)
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the Requirements
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<String>,

    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,

    /// Name for this Requirements (computer friendly)
    pub name: Option<String>,

    /// Name for this Requirements (human friendly)
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

    /// Natural language description of the requirements
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for Requirements (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this Requirements is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// Other set of Requirements this builds on
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<String>>,

    /// External artifact (rule/document etc. that) created this set of requirements
    pub reference: Option<Vec<String>>,

    /// Actor for these requirements
    pub actor: Option<Vec<String>>,

    /// Actual statement as markdown
    pub statement: Option<Vec<RequirementsStatement>>,
}
