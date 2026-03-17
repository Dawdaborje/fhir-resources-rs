//! FHIR R5 ActorDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// The ActorDefinition resource is used to describe an actor - a human or an application that plays a role in data exchange, and that may have obligations associated with the role the actor plays.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorDefinition {
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

    /// Canonical identifier for this actor definition, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the actor definition (business identifier)
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the actor definition
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<String>,

    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,

    /// Name for this actor definition (computer friendly)
    pub name: Option<String>,

    /// Name for this actor definition (human friendly)
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

    /// Natural language description of the actor
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for actor definition (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this actor definition is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// person | system
    #[serde(rename = "type")]
    pub r#type: String,

    /// Functionality associated with the actor
    pub documentation: Option<String>,

    /// Reference to more information about the actor
    pub reference: Option<Vec<String>>,

    /// CapabilityStatement for the actor (if applicable)
    pub capabilities: Option<String>,

    /// Definition of this actor in another context / IG
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<String>>,
}
