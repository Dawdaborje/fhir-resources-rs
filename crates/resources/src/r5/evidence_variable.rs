//! FHIR R5 EvidenceVariable Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// A defining factor of the EvidenceVariable
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Label for internal linking
    #[serde(rename = "linkId")]
    pub link_id: Option<String>,

    /// Natural language description of the characteristic
    pub description: Option<String>,

    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<Annotation>>,

    /// Whether the characteristic is an inclusion criterion or exclusion criterion
    pub exclude: Option<bool>,

    /// Defines the characteristic (without using type and value) by a Reference
    #[serde(rename = "definitionReference")]
    pub definition_reference: Option<Box<Reference>>,

    /// Defines the characteristic (without using type and value) by a Canonical
    #[serde(rename = "definitionCanonical")]
    pub definition_canonical: Option<String>,

    /// Defines the characteristic (without using type and value) by a CodeableConcept
    #[serde(rename = "definitionCodeableConcept")]
    pub definition_codeable_concept: Option<CodeableConcept>,

    /// Defines the characteristic (without using type and value) by an expression
    #[serde(rename = "definitionExpression")]
    pub definition_expression: Option<Expression>,

    /// Defines the characteristic (without using type and value) by an id
    #[serde(rename = "definitionId")]
    pub definition_id: Option<String>,

    /// Defines the characteristic using type and value
    #[serde(rename = "definitionByTypeAndValue")]
    pub definition_by_type_and_value:
        Option<EvidenceVariableCharacteristicDefinitionByTypeAndValue>,

    /// Used to specify how two or more characteristics are combined
    #[serde(rename = "definitionByCombination")]
    pub definition_by_combination: Option<EvidenceVariableCharacteristicDefinitionByCombination>,

    /// Number of occurrences meeting the characteristic
    #[serde(rename = "instancesQuantity")]
    pub instances_quantity: Option<Quantity>,

    #[serde(rename = "instancesRange")]
    pub instances_range: Option<Range>,

    /// Length of time in which the characteristic is met
    #[serde(rename = "durationQuantity")]
    pub duration_quantity: Option<Quantity>,

    #[serde(rename = "durationRange")]
    pub duration_range: Option<Range>,

    /// Timing in which the characteristic is determined
    #[serde(rename = "timeFromEvent")]
    pub time_from_event: Option<Vec<EvidenceVariableCharacteristicTimeFromEvent>>,
}

/// Defines the characteristic using type and value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCharacteristicDefinitionByTypeAndValue {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Expresses the type of characteristic
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Method for how the characteristic value was determined
    pub method: Option<Vec<CodeableConcept>>,

    /// Device used for determining characteristic
    pub device: Option<Box<Reference>>,

    /// Defines the characteristic when coupled with characteristic.type
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: bool,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,

    #[serde(rename = "valueRange")]
    pub value_range: Range,

    #[serde(rename = "valueReference")]
    pub value_reference: Box<Reference>,

    #[serde(rename = "valueId")]
    pub value_id: String,

    /// Reference point for valueQuantity or valueRange
    pub offset: Option<CodeableConcept>,
}

/// Used to specify how two or more characteristics are combined
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCharacteristicDefinitionByCombination {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// all-of | any-of | at-least | at-most | statistical | net-effect | dataset
    pub code: String,

    /// Provides the value of "n" when "at-least" or "at-most" codes are used
    pub threshold: Option<i32>,

    /// A defining factor of the characteristic
    pub characteristic: Vec<EvidenceVariableCharacteristic>,
}

/// Timing in which the characteristic is determined
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCharacteristicTimeFromEvent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Human readable description
    pub description: Option<String>,

    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<Annotation>>,

    /// The event used as a base point (reference point) in time
    #[serde(rename = "eventCodeableConcept")]
    pub event_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "eventReference")]
    pub event_reference: Option<Box<Reference>>,

    #[serde(rename = "eventDateTime")]
    pub event_date_time: Option<String>,

    #[serde(rename = "eventId")]
    pub event_id: Option<String>,

    /// Used to express the observation at a defined amount of time before or after the event
    pub quantity: Option<Quantity>,

    /// Used to express the observation within a period before and/or after the event
    pub range: Option<Range>,
}

/// A grouping for ordinal or polychotomous variables
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCategory {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Description of the grouping
    pub name: Option<String>,

    /// Definition of the grouping
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,

    #[serde(rename = "valueRange")]
    pub value_range: Option<Range>,
}

/// The EvidenceVariable resource describes an element that knowledge (Evidence) is about.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariable {
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

    /// Canonical identifier for this evidence variable, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the evidence variable
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the evidence variable
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<String>,

    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,

    /// Name for this evidence variable (computer friendly)
    pub name: Option<String>,

    /// Name for this evidence variable (human friendly)
    pub title: Option<String>,

    /// Title for use in informal contexts
    #[serde(rename = "shortTitle")]
    pub short_title: Option<String>,

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

    /// Natural language description of the evidence variable
    pub description: Option<String>,

    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<Annotation>>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Why this EvidenceVariable is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// When the resource was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the resource was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the resource is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// Who authored the content
    pub author: Option<Vec<ContactDetail>>,

    /// Who edited the content
    pub editor: Option<Vec<ContactDetail>>,

    /// Who reviewed the content
    pub reviewer: Option<Vec<ContactDetail>>,

    /// Who endorsed the content
    pub endorser: Option<Vec<ContactDetail>>,

    /// Additional documentation, citations, etc
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// Actual or conceptual
    pub actual: Option<bool>,

    /// A defining factor of the EvidenceVariable
    pub characteristic: Option<Vec<EvidenceVariableCharacteristic>>,

    /// continuous | dichotomous | ordinal | polychotomous
    pub handling: Option<String>,

    /// A grouping for ordinal or polychotomous variables
    pub category: Option<Vec<EvidenceVariableCategory>>,
}
