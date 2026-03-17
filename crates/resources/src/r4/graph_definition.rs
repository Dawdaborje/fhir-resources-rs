//! FHIR R4 GraphDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

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

    /// Path in the resource that contains the link
    pub path: Option<String>,

    /// Which slice (if profiled)
    #[serde(rename = "sliceName")]
    pub slice_name: Option<String>,

    /// Minimum occurrences for this link
    pub min: Option<i32>,

    /// Maximum occurrences for this link
    pub max: Option<String>,

    /// Why this link is specified
    pub description: Option<String>,

    /// Potential target for the link
    pub target: Option<Vec<GraphDefinitionLinkTarget>>,
}

/// Potential target for the link
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinitionLinkTarget {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of resource this link refers to
    #[serde(rename = "type")]
    pub r#type: String,

    /// Criteria for reverse lookup
    pub params: Option<String>,

    /// Profile for the target resource
    pub profile: Option<String>,

    /// Compartment Consistency Rules
    pub compartment: Option<Vec<GraphDefinitionLinkTargetCompartment>>,

    /// Additional links from target resource
    pub link: Option<Vec<GraphDefinitionLink>>,
}

/// Compartment Consistency Rules
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphDefinitionLinkTargetCompartment {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// condition | requirement
    #[serde(rename = "use")]
    pub r#use: String,

    /// Patient | Encounter | RelatedPerson | Practitioner | Device
    pub code: String,

    /// identical | matching | different | custom
    pub rule: String,

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

    /// Business version of the graph definition
    pub version: Option<String>,

    /// Name for this graph definition (computer friendly)
    pub name: String,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher (organization or individual)
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

    /// Type of resource at which the graph starts
    pub start: String,

    /// Profile on base resource
    pub profile: Option<String>,

    /// Links this graph makes rules about
    pub link: Option<Vec<GraphDefinitionLink>>,
}
