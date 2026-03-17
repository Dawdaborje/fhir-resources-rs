//! FHIR R4 RiskEvidenceSynthesis Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// What sample size was involved?
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskEvidenceSynthesisSampleSize {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Description of sample size
    pub description: Option<String>,

    /// How many studies?
    #[serde(rename = "numberOfStudies")]
    pub number_of_studies: Option<i32>,

    /// How many participants?
    #[serde(rename = "numberOfParticipants")]
    pub number_of_participants: Option<i32>,
}

/// What was the estimated risk
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskEvidenceSynthesisRiskEstimate {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Description of risk estimate
    pub description: Option<String>,

    /// Type of risk estimate
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Point estimate
    pub value: Option<f64>,

    /// What unit is the outcome described in?
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: Option<CodeableConcept>,

    /// Sample size for group measured
    #[serde(rename = "denominatorCount")]
    pub denominator_count: Option<i32>,

    /// Number with the outcome
    #[serde(rename = "numeratorCount")]
    pub numerator_count: Option<i32>,

    /// How precise the estimate is
    #[serde(rename = "precisionEstimate")]
    pub precision_estimate: Option<Vec<RiskEvidenceSynthesisRiskEstimatePrecisionEstimate>>,
}

/// How precise the estimate is
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskEvidenceSynthesisRiskEstimatePrecisionEstimate {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of precision estimate
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Level of confidence interval
    pub level: Option<f64>,

    /// Lower bound
    pub from: Option<f64>,

    /// Upper bound
    pub to: Option<f64>,
}

/// How certain is the risk
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskEvidenceSynthesisCertainty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Certainty rating
    pub rating: Option<Vec<CodeableConcept>>,

    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<Annotation>>,

    /// A component that contributes to the overall certainty
    #[serde(rename = "certaintySubcomponent")]
    pub certainty_subcomponent: Option<Vec<RiskEvidenceSynthesisCertaintyCertaintySubcomponent>>,
}

/// A component that contributes to the overall certainty
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskEvidenceSynthesisCertaintyCertaintySubcomponent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of subcomponent of certainty rating
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Subcomponent certainty rating
    pub rating: Option<Vec<CodeableConcept>>,

    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<Annotation>>,
}

/// The RiskEvidenceSynthesis resource describes the likelihood of an outcome in a population plus exposure state where the risk estimate is derived from a combination of research studies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskEvidenceSynthesis {
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

    /// Canonical identifier for this risk evidence synthesis, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the risk evidence synthesis
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the risk evidence synthesis
    pub version: Option<String>,

    /// Name for this risk evidence synthesis (computer friendly)
    pub name: Option<String>,

    /// Name for this risk evidence synthesis (human friendly)
    pub title: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the risk evidence synthesis
    pub description: Option<String>,

    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<Annotation>>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for risk evidence synthesis (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// When the risk evidence synthesis was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the risk evidence synthesis was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the risk evidence synthesis is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// The category of the EffectEvidenceSynthesis, such as Education, Treatment, Assessment, etc.
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

    /// Type of synthesis
    #[serde(rename = "synthesisType")]
    pub synthesis_type: Option<CodeableConcept>,

    /// Type of study
    #[serde(rename = "studyType")]
    pub study_type: Option<CodeableConcept>,

    /// What population?
    pub population: Box<Reference>,

    /// What exposure?
    pub exposure: Option<Box<Reference>>,

    /// What outcome?
    pub outcome: Box<Reference>,

    /// What sample size was involved?
    #[serde(rename = "sampleSize")]
    pub sample_size: Option<RiskEvidenceSynthesisSampleSize>,

    /// What was the estimated risk
    #[serde(rename = "riskEstimate")]
    pub risk_estimate: Option<RiskEvidenceSynthesisRiskEstimate>,

    /// How certain is the risk
    pub certainty: Option<Vec<RiskEvidenceSynthesisCertainty>>,
}
