//! FHIR R4B EvidenceVariable Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// What defines the members of the evidence element
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

    /// Natural language description of the characteristic
    pub description: Option<String>,

    /// What code or expression defines members?
    #[serde(rename = "definitionReference")]
    pub definition_reference: Box<Reference>,

    #[serde(rename = "definitionCanonical")]
    pub definition_canonical: String,

    #[serde(rename = "definitionCodeableConcept")]
    pub definition_codeable_concept: CodeableConcept,

    #[serde(rename = "definitionExpression")]
    pub definition_expression: Expression,

    /// Method used for describing characteristic
    pub method: Option<CodeableConcept>,

    /// Device used for determining characteristic
    pub device: Option<Box<Reference>>,

    /// Whether the characteristic includes or excludes members
    pub exclude: Option<bool>,

    /// Observation time from study start
    #[serde(rename = "timeFromStart")]
    pub time_from_start: Option<EvidenceVariableCharacteristicTimeFromStart>,

    /// mean | median | mean-of-mean | mean-of-median | median-of-mean | median-of-median
    #[serde(rename = "groupMeasure")]
    pub group_measure: Option<String>,
}

/// Observation time from study start
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableCharacteristicTimeFromStart {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Human readable description
    pub description: Option<String>,

    /// Used to express the observation at a defined amount of time after the study start
    pub quantity: Option<Quantity>,

    /// Used to express the observation within a period after the study start
    pub range: Option<Range>,

    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<Annotation>>,
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

    /// Name for this evidence variable (computer friendly)
    pub name: Option<String>,

    /// Name for this evidence variable (human friendly)
    pub title: Option<String>,

    /// Title for use in informal contexts
    #[serde(rename = "shortTitle")]
    pub short_title: Option<String>,

    /// Subordinate title of the EvidenceVariable
    pub subtitle: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// Date last changed
    pub date: Option<String>,

    /// Natural language description of the evidence variable
    pub description: Option<String>,

    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<Annotation>>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Who authored the content
    pub author: Option<Vec<ContactDetail>>,

    /// Who edited the content
    pub editor: Option<Vec<ContactDetail>>,

    /// Who reviewed the content
    pub reviewer: Option<Vec<ContactDetail>>,

    /// Who endorsed the content
    pub endorser: Option<Vec<ContactDetail>>,

    /// Additional documentation, citations, etc.
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// Actual or conceptual
    pub actual: Option<bool>,

    /// intersection | union
    #[serde(rename = "characteristicCombination")]
    pub characteristic_combination: Option<String>,

    /// What defines the members of the evidence element
    pub characteristic: Option<Vec<EvidenceVariableCharacteristic>>,

    /// continuous | dichotomous | ordinal | polychotomous
    pub handling: Option<String>,

    /// A grouping for ordinal or polychotomous variables
    pub category: Option<Vec<EvidenceVariableCategory>>,
}
