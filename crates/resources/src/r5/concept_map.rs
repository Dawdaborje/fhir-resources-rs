//! FHIR R5 ConceptMap Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Additional properties of the mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapProperty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Identifies the property on the mappings, and when referred to in the $translate operation
    pub code: String,

    /// Formal identifier for the property
    pub uri: Option<String>,

    /// Why the property is defined, and/or what it conveys
    pub description: Option<String>,

    /// Coding | string | integer | boolean | dateTime | decimal | code
    #[serde(rename = "type")]
    pub r#type: String,

    /// The CodeSystem from which code values come
    pub system: Option<String>,
}

/// Definition of an additional attribute to act as a data source or target
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapAdditionalAttribute {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Identifies this additional attribute through this resource
    pub code: String,

    /// Formal identifier for the data element referred to in this attribte
    pub uri: Option<String>,

    /// Why the additional attribute is defined, and/or what the data element it refers to is
    pub description: Option<String>,

    /// code | Coding | string | boolean | Quantity
    #[serde(rename = "type")]
    pub r#type: String,
}

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

    /// Target system that the concepts are to be mapped to
    pub target: Option<String>,

    /// Mappings for a concept from the source set
    pub element: Vec<ConceptMapGroupElement>,

    /// What to do when there is no mapping target for the source concept and ConceptMap.group.element.noMap is not true
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

    /// Identifies the set of concepts being mapped
    #[serde(rename = "valueSet")]
    pub value_set: Option<String>,

    /// No mapping to a target concept for this source concept
    #[serde(rename = "noMap")]
    pub no_map: Option<bool>,

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

    /// Identifies the set of target concepts
    #[serde(rename = "valueSet")]
    pub value_set: Option<String>,

    /// related-to | equivalent | source-is-narrower-than-target | source-is-broader-than-target | not-related-to
    pub relationship: String,

    /// Description of status/issues in mapping
    pub comment: Option<String>,

    /// Property value for the source -> target mapping
    pub property: Option<Vec<ConceptMapGroupElementTargetProperty>>,

    /// Other properties required for this mapping
    #[serde(rename = "dependsOn")]
    pub depends_on: Option<Vec<ConceptMapGroupElementTargetDependsOn>>,

    /// Other data elements that this mapping also produces
    pub product: Option<Vec<ConceptMapGroupElementTargetDependsOn>>,
}

/// Property value for the source -> target mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMapGroupElementTargetProperty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reference to ConceptMap.property.code
    pub code: String,

    /// Value of the property for this concept
    #[serde(rename = "valueCoding")]
    pub value_coding: Coding,

    #[serde(rename = "valueString")]
    pub value_string: String,

    #[serde(rename = "valueInteger")]
    pub value_integer: i32,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: bool,

    #[serde(rename = "valueDateTime")]
    pub value_date_time: String,

    #[serde(rename = "valueDecimal")]
    pub value_decimal: f64,

    #[serde(rename = "valueCode")]
    pub value_code: String,
}

/// Other properties required for this mapping
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

    /// A reference to a mapping attribute defined in ConceptMap.additionalAttribute
    pub attribute: String,

    /// Value of the referenced data element
    #[serde(rename = "valueCode")]
    pub value_code: Option<String>,

    #[serde(rename = "valueCoding")]
    pub value_coding: Option<Coding>,

    #[serde(rename = "valueString")]
    pub value_string: Option<String>,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<bool>,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,

    /// The mapping depends on a data element with a value from this value set
    #[serde(rename = "valueSet")]
    pub value_set: Option<String>,
}

/// What to do when there is no mapping target for the source concept and ConceptMap.group.element.noMap is not true
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

    /// use-source-code | fixed | other-map
    pub mode: String,

    /// Fixed code when mode = fixed
    pub code: Option<String>,

    /// Display for the code
    pub display: Option<String>,

    /// Fixed code set when mode = fixed
    #[serde(rename = "valueSet")]
    pub value_set: Option<String>,

    /// related-to | equivalent | source-is-narrower-than-target | source-is-broader-than-target | not-related-to
    pub relationship: Option<String>,

    /// canonical reference to an additional ConceptMap to use for mapping if the source concept is unmapped
    #[serde(rename = "otherMap")]
    pub other_map: Option<String>,
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
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the concept map
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<String>,

    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,

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

    /// Name of the publisher/steward (organization or individual)
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

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// When the ConceptMap was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the ConceptMap was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the ConceptMap is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// E.g. Education, Treatment, Assessment, etc
    pub topic: Option<Vec<CodeableConcept>>,

    /// Who authored the ConceptMap
    pub author: Option<Vec<ContactDetail>>,

    /// Who edited the ConceptMap
    pub editor: Option<Vec<ContactDetail>>,

    /// Who reviewed the ConceptMap
    pub reviewer: Option<Vec<ContactDetail>>,

    /// Who endorsed the ConceptMap
    pub endorser: Option<Vec<ContactDetail>>,

    /// Additional documentation, citations, etc
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// Additional properties of the mapping
    pub property: Option<Vec<ConceptMapProperty>>,

    /// Definition of an additional attribute to act as a data source or target
    #[serde(rename = "additionalAttribute")]
    pub additional_attribute: Option<Vec<ConceptMapAdditionalAttribute>>,

    /// The source value set that contains the concepts that are being mapped
    #[serde(rename = "sourceScopeUri")]
    pub source_scope_uri: Option<String>,

    #[serde(rename = "sourceScopeCanonical")]
    pub source_scope_canonical: Option<String>,

    /// The target value set which provides context for the mappings
    #[serde(rename = "targetScopeUri")]
    pub target_scope_uri: Option<String>,

    #[serde(rename = "targetScopeCanonical")]
    pub target_scope_canonical: Option<String>,

    /// Same source and target systems
    pub group: Option<Vec<ConceptMapGroup>>,
}
