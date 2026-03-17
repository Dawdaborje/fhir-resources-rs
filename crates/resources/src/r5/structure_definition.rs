//! FHIR R5 StructureDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// External specification that the content is mapped to
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureDefinitionMapping {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Internal id when this mapping is used
    pub identity: String,

    /// Identifies what this mapping refers to
    pub uri: Option<String>,

    /// Names what this mapping refers to
    pub name: Option<String>,

    /// Versions, Issues, Scope limitations etc
    pub comment: Option<String>,
}

/// If an extension, where it can be used in instances
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureDefinitionContext {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// fhirpath | element | extension
    #[serde(rename = "type")]
    pub r#type: String,

    /// Where the extension can be used in instances
    pub expression: String,
}

/// Snapshot view of the structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureDefinitionSnapshot {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Definition of elements in the resource (if no StructureDefinition)
    pub element: Vec<ElementDefinition>,
}

/// Differential view of the structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureDefinitionDifferential {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Definition of elements in the resource (if no StructureDefinition)
    pub element: Vec<ElementDefinition>,
}

/// A definition of a FHIR structure. This resource is used to describe the underlying resources, data types defined in FHIR, and also for describing extensions and constraints on resources and data ty...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureDefinition {
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

    /// Canonical identifier for this structure definition, represented as a URI (globally unique)
    pub url: String,

    /// Additional identifier for the structure definition
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the structure definition
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<String>,

    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,

    /// Name for this structure definition (computer friendly)
    pub name: String,

    /// Name for this structure definition (human friendly)
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

    /// Natural language description of the structure definition
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for structure definition (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this structure definition is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// Assist with indexing and finding
    pub keyword: Option<Vec<Coding>>,

    /// FHIR Version this StructureDefinition targets
    #[serde(rename = "fhirVersion")]
    pub fhir_version: Option<String>,

    /// External specification that the content is mapped to
    pub mapping: Option<Vec<StructureDefinitionMapping>>,

    /// primitive-type | complex-type | resource | logical
    pub kind: String,

    /// Whether the structure is abstract
    #[serde(rename = "abstract")]
    pub r#abstract: bool,

    /// If an extension, where it can be used in instances
    pub context: Option<Vec<StructureDefinitionContext>>,

    /// FHIRPath invariants - when the extension can be used
    #[serde(rename = "contextInvariant")]
    pub context_invariant: Option<Vec<String>>,

    /// Type defined or constrained by this structure
    #[serde(rename = "type")]
    pub r#type: String,

    /// Definition that this type is constrained/specialized from
    #[serde(rename = "baseDefinition")]
    pub base_definition: Option<String>,

    /// specialization | constraint - How relates to base definition
    pub derivation: Option<String>,

    /// Snapshot view of the structure
    pub snapshot: Option<StructureDefinitionSnapshot>,

    /// Differential view of the structure
    pub differential: Option<StructureDefinitionDifferential>,
}
