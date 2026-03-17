//! FHIR R5 StructureMap Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Structure Definition used by this map
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapStructure {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Canonical reference to structure definition
    pub url: String,

    /// source | queried | target | produced
    pub mode: String,

    /// Name for type in this map
    pub alias: Option<String>,

    /// Documentation on use of structure
    pub documentation: Option<String>,
}

/// Definition of the constant value used in the map rules
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapConst {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Constant name
    pub name: Option<String>,

    /// FHIRPath exression - value of the constant
    pub value: Option<String>,
}

/// Named sections for reader convenience
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroup {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Human-readable label
    pub name: String,

    /// Another group that this group adds rules to
    pub extends: Option<String>,

    /// types | type-and-types
    #[serde(rename = "typeMode")]
    pub type_mode: Option<String>,

    /// Additional description/explanation for group
    pub documentation: Option<String>,

    /// Named instance provided when invoking the map
    pub input: Vec<StructureMapGroupInput>,

    /// Transform Rule from source to target
    pub rule: Option<Vec<StructureMapGroupRule>>,
}

/// Named instance provided when invoking the map
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupInput {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name for this instance of data
    pub name: String,

    /// Type for this instance of data
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// source | target
    pub mode: String,

    /// Documentation for this instance of data
    pub documentation: Option<String>,
}

/// Transform Rule from source to target
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRule {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name of the rule for internal references
    pub name: Option<String>,

    /// Source inputs to the mapping
    pub source: Vec<StructureMapGroupRuleSource>,

    /// Content to create because of this mapping rule
    pub target: Option<Vec<StructureMapGroupRuleTarget>>,

    /// Rules contained in this rule
    pub rule: Option<Vec<StructureMapGroupRule>>,

    /// Which other rules to apply in the context of this rule
    pub dependent: Option<Vec<StructureMapGroupRuleDependent>>,

    /// Documentation for this instance of data
    pub documentation: Option<String>,
}

/// Source inputs to the mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRuleSource {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type or variable this rule applies to
    pub context: String,

    /// Specified minimum cardinality
    pub min: Option<i32>,

    /// Specified maximum cardinality (number or *)
    pub max: Option<String>,

    /// Rule only applies if source has this type
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// Default value if no value exists
    #[serde(rename = "defaultValue")]
    pub default_value: Option<String>,

    /// Optional field for this source
    pub element: Option<String>,

    /// first | not_first | last | not_last | only_one
    #[serde(rename = "listMode")]
    pub list_mode: Option<String>,

    /// Named context for field, if a field is specified
    pub variable: Option<String>,

    /// FHIRPath expression - must be true or the rule does not apply
    pub condition: Option<String>,

    /// FHIRPath expression - must be true or the mapping engine throws an error instead of completing
    pub check: Option<String>,

    /// Message to put in log if source exists (FHIRPath)
    #[serde(rename = "logMessage")]
    pub log_message: Option<String>,
}

/// Content to create because of this mapping rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRuleTarget {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Variable this rule applies to
    pub context: Option<String>,

    /// Field to create in the context
    pub element: Option<String>,

    /// Named context for field, if desired, and a field is specified
    pub variable: Option<String>,

    /// first | share | last | single
    #[serde(rename = "listMode")]
    pub list_mode: Option<Vec<String>>,

    /// Internal rule reference for shared list items
    #[serde(rename = "listRuleId")]
    pub list_rule_id: Option<String>,

    /// create | copy +
    pub transform: Option<String>,

    /// Parameters to the transform
    pub parameter: Option<Vec<StructureMapGroupRuleTargetParameter>>,
}

/// Parameters to the transform
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRuleTargetParameter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Parameter value - variable or literal
    pub value: serde_json::Value,
}

/// Which other rules to apply in the context of this rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMapGroupRuleDependent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name of a rule or group to apply
    pub name: String,

    /// Parameter to pass to the rule or group
    pub parameter: Vec<StructureMapGroupRuleTargetParameter>,
}

/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureMap {
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

    /// Canonical identifier for this structure map, represented as a URI (globally unique)
    pub url: String,

    /// Additional identifier for the structure map
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the structure map
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithm")]
    pub version_algorithm: Option<serde_json::Value>,

    /// Name for this structure map (computer friendly)
    pub name: String,

    /// Name for this structure map (human friendly)
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

    /// Natural language description of the structure map
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for structure map (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this structure map is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// Structure Definition used by this map
    pub structure: Option<Vec<StructureMapStructure>>,

    /// Other maps used by this map (canonical URLs)
    pub import: Option<Vec<String>>,

    /// Definition of the constant value used in the map rules
    #[serde(rename = "const")]
    pub r#const: Option<Vec<StructureMapConst>>,

    /// Named sections for reader convenience
    pub group: Vec<StructureMapGroup>,
}
