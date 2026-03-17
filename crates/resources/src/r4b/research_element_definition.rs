//! FHIR R4B ResearchElementDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// What defines the members of the research element
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchElementDefinitionCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// What code or expression defines members?
    #[serde(rename = "definitionCodeableConcept")]
    pub definition_codeable_concept: CodeableConcept,

    #[serde(rename = "definitionCanonical")]
    pub definition_canonical: String,

    #[serde(rename = "definitionExpression")]
    pub definition_expression: Expression,

    #[serde(rename = "definitionDataRequirement")]
    pub definition_data_requirement: DataRequirement,

    /// What code/value pairs define members?
    #[serde(rename = "usageContext")]
    pub usage_context: Option<Vec<UsageContext>>,

    /// Whether the characteristic includes or excludes members
    pub exclude: Option<bool>,

    /// What unit is the outcome described in?
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: Option<CodeableConcept>,

    /// What time period does the study cover
    #[serde(rename = "studyEffectiveDescription")]
    pub study_effective_description: Option<String>,

    /// What time period does the study cover
    #[serde(rename = "studyEffectiveDateTime")]
    pub study_effective_date_time: Option<String>,

    #[serde(rename = "studyEffectivePeriod")]
    pub study_effective_period: Option<Period>,

    #[serde(rename = "studyEffectiveDuration")]
    pub study_effective_duration: Option<Duration>,

    #[serde(rename = "studyEffectiveTiming")]
    pub study_effective_timing: Option<Timing>,

    /// Observation time from study start
    #[serde(rename = "studyEffectiveTimeFromStart")]
    pub study_effective_time_from_start: Option<Duration>,

    /// mean | median | mean-of-mean | mean-of-median | median-of-mean | median-of-median
    #[serde(rename = "studyEffectiveGroupMeasure")]
    pub study_effective_group_measure: Option<String>,

    /// What time period do participants cover
    #[serde(rename = "participantEffectiveDescription")]
    pub participant_effective_description: Option<String>,

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
    #[serde(rename = "participantEffectiveTimeFromStart")]
    pub participant_effective_time_from_start: Option<Duration>,

    /// mean | median | mean-of-mean | mean-of-median | median-of-mean | median-of-median
    #[serde(rename = "participantEffectiveGroupMeasure")]
    pub participant_effective_group_measure: Option<String>,
}

/// The ResearchElementDefinition resource describes a "PICO" element that knowledge (evidence, assertion, recommendation) is about.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchElementDefinition {
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

    /// Canonical identifier for this research element definition, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the research element definition
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the research element definition
    pub version: Option<String>,

    /// Name for this research element definition (computer friendly)
    pub name: Option<String>,

    /// Name for this research element definition (human friendly)
    pub title: Option<String>,

    /// Title for use in informal contexts
    #[serde(rename = "shortTitle")]
    pub short_title: Option<String>,

    /// Subordinate title of the ResearchElementDefinition
    pub subtitle: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    #[serde(rename = "subjectCodeableConcept")]
    pub subject_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "subjectReference")]
    pub subject_reference: Option<Box<Reference>>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the research element definition
    pub description: Option<String>,

    /// Used for footnotes or explanatory notes
    pub comment: Option<Vec<String>>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for research element definition (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this research element definition is defined
    pub purpose: Option<String>,

    /// Describes the clinical usage of the ResearchElementDefinition
    pub usage: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// When the research element definition was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the research element definition was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the research element definition is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// The category of the ResearchElementDefinition, such as Education, Treatment, Assessment, etc.
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

    /// Logic used by the ResearchElementDefinition
    pub library: Option<Vec<String>>,

    /// population | exposure | outcome
    #[serde(rename = "type")]
    pub r#type: String,

    /// dichotomous | continuous | descriptive
    #[serde(rename = "variableType")]
    pub variable_type: Option<String>,

    /// What defines the members of the research element
    pub characteristic: Vec<ResearchElementDefinitionCharacteristic>,
}
