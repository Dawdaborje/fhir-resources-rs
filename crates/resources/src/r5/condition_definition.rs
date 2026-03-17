//! FHIR R5 ConditionDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Observations particularly relevant to this condition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionDefinitionObservation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Category that is relevant
    pub category: Option<CodeableConcept>,

    /// Code for relevant Observation
    pub code: Option<CodeableConcept>,
}

/// Medications particularly relevant for this condition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionDefinitionMedication {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Category that is relevant
    pub category: Option<CodeableConcept>,

    /// Code for relevant Medication
    pub code: Option<CodeableConcept>,
}

/// Observation that suggets this condition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionDefinitionPrecondition {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// sensitive | specific
    #[serde(rename = "type")]
    pub r#type: String,

    /// Code for relevant Observation
    pub code: CodeableConcept,

    /// Value of Observation
    pub value: Option<serde_json::Value>,
}

/// Questionnaire for this condition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionDefinitionQuestionnaire {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// preadmit | diff-diagnosis | outcome
    pub purpose: String,

    /// Specific Questionnaire
    pub reference: Box<Reference>,
}

/// Plan that is appropriate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionDefinitionPlan {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Use for the plan
    pub role: Option<CodeableConcept>,

    /// The actual plan
    pub reference: Box<Reference>,
}

/// A definition of a condition and information relevant to managing it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionDefinition {
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

    /// Canonical identifier for this condition definition, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the condition definition
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the condition definition
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithm")]
    pub version_algorithm: Option<serde_json::Value>,

    /// Name for this condition definition (computer friendly)
    pub name: Option<String>,

    /// Name for this condition definition (human friendly)
    pub title: Option<String>,

    /// Subordinate title of the event definition
    pub subtitle: Option<String>,

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

    /// Natural language description of the condition definition
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for condition definition (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Identification of the condition, problem or diagnosis
    pub code: CodeableConcept,

    /// Subjective severity of condition
    pub severity: Option<CodeableConcept>,

    /// Anatomical location, if relevant
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,

    /// Stage/grade, usually assessed formally
    pub stage: Option<CodeableConcept>,

    /// Whether Severity is appropriate
    #[serde(rename = "hasSeverity")]
    pub has_severity: Option<bool>,

    /// Whether bodySite is appropriate
    #[serde(rename = "hasBodySite")]
    pub has_body_site: Option<bool>,

    /// Whether stage is appropriate
    #[serde(rename = "hasStage")]
    pub has_stage: Option<bool>,

    /// Formal Definition for the condition
    pub definition: Option<Vec<String>>,

    /// Observations particularly relevant to this condition
    pub observation: Option<Vec<ConditionDefinitionObservation>>,

    /// Medications particularly relevant for this condition
    pub medication: Option<Vec<ConditionDefinitionMedication>>,

    /// Observation that suggets this condition
    pub precondition: Option<Vec<ConditionDefinitionPrecondition>>,

    /// Appropriate team for this condition
    pub team: Option<Vec<Box<Reference>>>,

    /// Questionnaire for this condition
    pub questionnaire: Option<Vec<ConditionDefinitionQuestionnaire>>,

    /// Plan that is appropriate
    pub plan: Option<Vec<ConditionDefinitionPlan>>,
}
