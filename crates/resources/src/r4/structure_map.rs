//! FHIR R4 StructureMap Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
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

    /// none | types | type-and-types
    #[serde(rename = "typeMode")]
    pub type_mode: String,

    /// Additional description/explanation for group
    pub documentation: Option<String>,

    /// Named instance provided when invoking the map
    pub input: Vec<StructureMapGroupInput>,

    /// Transform Rule from source to target
    pub rule: Vec<StructureMapGroupRule>,
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
    pub name: String,

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
    #[serde(rename = "defaultValueBase64Binary")]
    pub default_value_base64binary: Option<String>,

    #[serde(rename = "defaultValueBoolean")]
    pub default_value_boolean: Option<bool>,

    #[serde(rename = "defaultValueCanonical")]
    pub default_value_canonical: Option<String>,

    #[serde(rename = "defaultValueCode")]
    pub default_value_code: Option<String>,

    #[serde(rename = "defaultValueDate")]
    pub default_value_date: Option<String>,

    #[serde(rename = "defaultValueDateTime")]
    pub default_value_date_time: Option<String>,

    #[serde(rename = "defaultValueDecimal")]
    pub default_value_decimal: Option<f64>,

    #[serde(rename = "defaultValueId")]
    pub default_value_id: Option<String>,

    #[serde(rename = "defaultValueInstant")]
    pub default_value_instant: Option<String>,

    #[serde(rename = "defaultValueInteger")]
    pub default_value_integer: Option<i32>,

    #[serde(rename = "defaultValueMarkdown")]
    pub default_value_markdown: Option<String>,

    #[serde(rename = "defaultValueOid")]
    pub default_value_oid: Option<String>,

    #[serde(rename = "defaultValuePositiveInt")]
    pub default_value_positive_int: Option<i32>,

    #[serde(rename = "defaultValueString")]
    pub default_value_string: Option<String>,

    #[serde(rename = "defaultValueTime")]
    pub default_value_time: Option<String>,

    #[serde(rename = "defaultValueUnsignedInt")]
    pub default_value_unsigned_int: Option<u32>,

    #[serde(rename = "defaultValueUri")]
    pub default_value_uri: Option<String>,

    #[serde(rename = "defaultValueUrl")]
    pub default_value_url: Option<String>,

    #[serde(rename = "defaultValueUuid")]
    pub default_value_uuid: Option<String>,

    #[serde(rename = "defaultValueAddress")]
    pub default_value_address: Option<Address>,

    #[serde(rename = "defaultValueAge")]
    pub default_value_age: Option<Age>,

    #[serde(rename = "defaultValueAnnotation")]
    pub default_value_annotation: Option<Annotation>,

    #[serde(rename = "defaultValueAttachment")]
    pub default_value_attachment: Option<Attachment>,

    #[serde(rename = "defaultValueCodeableConcept")]
    pub default_value_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "defaultValueCoding")]
    pub default_value_coding: Option<Coding>,

    #[serde(rename = "defaultValueContactPoint")]
    pub default_value_contact_point: Option<ContactPoint>,

    #[serde(rename = "defaultValueCount")]
    pub default_value_count: Option<Count>,

    #[serde(rename = "defaultValueDistance")]
    pub default_value_distance: Option<Distance>,

    #[serde(rename = "defaultValueDuration")]
    pub default_value_duration: Option<Duration>,

    #[serde(rename = "defaultValueHumanName")]
    pub default_value_human_name: Option<HumanName>,

    #[serde(rename = "defaultValueIdentifier")]
    pub default_value_identifier: Option<Box<Identifier>>,

    #[serde(rename = "defaultValueMoney")]
    pub default_value_money: Option<Money>,

    #[serde(rename = "defaultValuePeriod")]
    pub default_value_period: Option<Period>,

    #[serde(rename = "defaultValueQuantity")]
    pub default_value_quantity: Option<Quantity>,

    #[serde(rename = "defaultValueRange")]
    pub default_value_range: Option<Range>,

    #[serde(rename = "defaultValueRatio")]
    pub default_value_ratio: Option<Ratio>,

    #[serde(rename = "defaultValueReference")]
    pub default_value_reference: Option<Box<Reference>>,

    #[serde(rename = "defaultValueSampledData")]
    pub default_value_sampled_data: Option<SampledData>,

    #[serde(rename = "defaultValueSignature")]
    pub default_value_signature: Option<Signature>,

    #[serde(rename = "defaultValueTiming")]
    pub default_value_timing: Option<Timing>,

    #[serde(rename = "defaultValueContactDetail")]
    pub default_value_contact_detail: Option<ContactDetail>,

    #[serde(rename = "defaultValueContributor")]
    pub default_value_contributor: Option<Contributor>,

    #[serde(rename = "defaultValueDataRequirement")]
    pub default_value_data_requirement: Option<DataRequirement>,

    #[serde(rename = "defaultValueExpression")]
    pub default_value_expression: Option<Expression>,

    #[serde(rename = "defaultValueParameterDefinition")]
    pub default_value_parameter_definition: Option<ParameterDefinition>,

    #[serde(rename = "defaultValueRelatedArtifact")]
    pub default_value_related_artifact: Option<RelatedArtifact>,

    #[serde(rename = "defaultValueTriggerDefinition")]
    pub default_value_trigger_definition: Option<TriggerDefinition>,

    #[serde(rename = "defaultValueUsageContext")]
    pub default_value_usage_context: Option<UsageContext>,

    #[serde(rename = "defaultValueDosage")]
    pub default_value_dosage: Option<Dosage>,

    #[serde(rename = "defaultValueMeta")]
    pub default_value_meta: Option<Meta>,

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

    /// Type or variable this rule applies to
    pub context: Option<String>,

    /// type | variable
    #[serde(rename = "contextType")]
    pub context_type: Option<String>,

    /// Field to create in the context
    pub element: Option<String>,

    /// Named context for field, if desired, and a field is specified
    pub variable: Option<String>,

    /// first | share | last | collate
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
    #[serde(rename = "valueId")]
    pub value_id: String,

    #[serde(rename = "valueString")]
    pub value_string: String,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: bool,

    #[serde(rename = "valueInteger")]
    pub value_integer: i32,

    #[serde(rename = "valueDecimal")]
    pub value_decimal: f64,
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

    /// Variable to pass to the rule or group
    pub variable: Vec<String>,
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

    /// Name of the publisher (organization or individual)
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

    /// Structure Definition used by this map
    pub structure: Option<Vec<StructureMapStructure>>,

    /// Other maps used by this map (canonical URLs)
    pub import: Option<Vec<String>>,

    /// Named sections for reader convenience
    pub group: Vec<StructureMapGroup>,
}
