//! FHIR R4 EvidenceVariable Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
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

    #[serde(rename = "definitionDataRequirement")]
    pub definition_data_requirement: DataRequirement,

    #[serde(rename = "definitionTriggerDefinition")]
    pub definition_trigger_definition: TriggerDefinition,

    /// What code/value pairs define members?
    #[serde(rename = "usageContext")]
    pub usage_context: Option<Vec<UsageContext>>,

    /// Whether the characteristic includes or excludes members
    pub exclude: Option<bool>,

    /// What time period do participants cover
    #[serde(rename = "participantEffectiveDateTime")]
    pub participant_effective_date_time: Option<String>,

    #[serde(rename = "participantEffectivePeriod")]
    pub participant_effective_period: Option<Period>,

    #[serde(rename = "participantEffectiveDuration")]
    pub participant_effective_duration: Option<Duration>,

    #[serde(rename = "participantEffectiveTiming")]
    pub participant_effective_timing: Option<Timing>,

    /// Observation time from study start
    #[serde(rename = "timeFromStart")]
    pub time_from_start: Option<Duration>,

    /// mean | median | mean-of-mean | mean-of-median | median-of-mean | median-of-median
    #[serde(rename = "groupMeasure")]
    pub group_measure: Option<String>,
}

/// The EvidenceVariable resource describes a "PICO" element that knowledge (evidence, assertion, recommendation) is about.
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

    /// Name of the publisher (organization or individual)
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

    /// Intended jurisdiction for evidence variable (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// When the evidence variable was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the evidence variable was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the evidence variable is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// The category of the EvidenceVariable, such as Education, Treatment, Assessment, etc.
    pub topic: Option<Vec<CodeableConcept>>,

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

    /// dichotomous | continuous | descriptive
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// What defines the members of the evidence element
    pub characteristic: Vec<EvidenceVariableCharacteristic>,
}
