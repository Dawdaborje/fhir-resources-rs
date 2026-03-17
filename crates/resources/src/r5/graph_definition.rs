//! FHIR R5 GraphDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Potential target for the link
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinitionNode {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Internal ID - target for link references
    #[serde(rename = "nodeId")]
    pub node_id: String,

    /// Why this node is specified
    pub description: Option<String>,

    /// Type of resource this link refers to
    #[serde(rename = "type")]
    pub r#type: String,

    /// Profile for the target resource
    pub profile: Option<String>,
}

/// Links this graph makes rules about
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinitionLink {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Why this link is specified
    pub description: Option<String>,

    /// Minimum occurrences for this link
    pub min: Option<i32>,

    /// Maximum occurrences for this link
    pub max: Option<String>,

    /// Source Node for this link
    #[serde(rename = "sourceId")]
    pub source_id: String,

    /// Path in the resource that contains the link
    pub path: Option<String>,

    /// Which slice (if profiled)
    #[serde(rename = "sliceName")]
    pub slice_name: Option<String>,

    /// Target Node for this link
    #[serde(rename = "targetId")]
    pub target_id: String,

    /// Criteria for reverse lookup
    pub params: Option<String>,

    /// Compartment Consistency Rules
    pub compartment: Option<Vec<GraphDefinitionLinkCompartment>>,
}

/// Compartment Consistency Rules
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinitionLinkCompartment {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// where | requires
    #[serde(rename = "use")]
    pub r#use: String,

    /// identical | matching | different | custom
    pub rule: String,

    /// Patient | Encounter | RelatedPerson | Practitioner | Device | EpisodeOfCare
    pub code: String,

    /// Custom rule, as a FHIRPath expression
    pub expression: Option<String>,

    /// Documentation for FHIRPath expression
    pub description: Option<String>,
}

/// A formal computable definition of a graph of resources - that is, a coherent set of resources that form a graph by following references. The Graph Definition resource defines a set and makes rules ...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinition {
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

    /// Canonical identifier for this graph definition, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the GraphDefinition (business identifier)
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the graph definition
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<String>,

    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,

    /// Name for this graph definition (computer friendly)
    pub name: String,

    /// Name for this graph definition (human friendly)
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

    /// Natural language description of the graph definition
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for graph definition (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this graph definition is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// Starting Node
    pub start: Option<String>,

    /// Potential target for the link
    pub node: Option<Vec<GraphDefinitionNode>>,

    /// Links this graph makes rules about
    pub link: Option<Vec<GraphDefinitionLink>>,
}
