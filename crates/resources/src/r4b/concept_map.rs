//! FHIR R4B ConceptMap Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Same source and target systems
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroup {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Source system where concepts to be mapped are defined
    pub source: Option<String>,

    /// Specific version of the code system
    #[serde(rename = "sourceVersion")]
    pub source_version: Option<String>,

    /// Target system that the concepts are to be mapped to
    pub target: Option<String>,

    /// Specific version of the code system
    #[serde(rename = "targetVersion")]
    pub target_version: Option<String>,

    /// Mappings for a concept from the source set
    pub element: Vec<ConceptMapGroupElement>,

    /// What to do when there is no mapping for the source concept
    pub unmapped: Option<ConceptMapGroupUnmapped>,
}

/// Mappings for a concept from the source set
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupElement {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Identifies element being mapped
    pub code: Option<String>,

    /// Display for the code
    pub display: Option<String>,

    /// Concept in target system for element
    pub target: Option<Vec<ConceptMapGroupElementTarget>>,
}

/// Concept in target system for element
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupElementTarget {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code that identifies the target element
    pub code: Option<String>,

    /// Display for the code
    pub display: Option<String>,

    /// relatedto | equivalent | equal | wider | subsumes | narrower | specializes | inexact | unmatched | disjoint
    pub equivalence: String,

    /// Description of status/issues in mapping
    pub comment: Option<String>,

    /// Other elements required for this mapping (from context)
    #[serde(rename = "dependsOn")]
    pub depends_on: Option<Vec<ConceptMapGroupElementTargetDependsOn>>,

    /// Other concepts that this mapping also produces
    pub product: Option<Vec<ConceptMapGroupElementTargetDependsOn>>,
}

/// Other elements required for this mapping (from context)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupElementTargetDependsOn {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reference to property mapping depends on
    pub property: String,

    /// Code System (if necessary)
    pub system: Option<String>,

    /// Value of the referenced element
    pub value: String,

    /// Display for the code (if value is a code)
    pub display: Option<String>,
}

/// What to do when there is no mapping for the source concept
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupUnmapped {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// provided | fixed | other-map
    pub mode: String,

    /// Fixed code when mode = fixed
    pub code: Option<String>,

    /// Display for the code
    pub display: Option<String>,

    /// canonical reference to an additional ConceptMap to use for mapping if the source concept is unmapped
    pub url: Option<String>,
}

/// A statement of relationships from one set of concepts to one or more other concepts - either concepts in code systems, or data element/data element concepts, or classes in class models.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMap {
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

    /// Canonical identifier for this concept map, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the concept map
    pub identifier: Option<Box<Identifier>>,

    /// Business version of the concept map
    pub version: Option<String>,

    /// Name for this concept map (computer friendly)
    pub name: Option<String>,

    /// Name for this concept map (human friendly)
    pub title: Option<String>,

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

    /// Natural language description of the concept map
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for concept map (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this concept map is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// The source value set that contains the concepts that are being mapped
    #[serde(rename = "sourceUri")]
    pub source_uri: Option<String>,

    #[serde(rename = "sourceCanonical")]
    pub source_canonical: Option<String>,

    /// The target value set which provides context for the mappings
    #[serde(rename = "targetUri")]
    pub target_uri: Option<String>,

    #[serde(rename = "targetCanonical")]
    pub target_canonical: Option<String>,

    /// Same source and target systems
    pub group: Option<Vec<ConceptMapGroup>>,
}
