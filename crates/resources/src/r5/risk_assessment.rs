//! FHIR R5 RiskAssessment Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Outcome predicted
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskAssessmentPrediction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Possible outcome for the subject
    pub outcome: Option<CodeableConcept>,

    /// Likelihood of specified outcome
    pub probability: Option<serde_json::Value>,

    /// Likelihood of specified outcome as a qualitative value
    #[serde(rename = "qualitativeRisk")]
    pub qualitative_risk: Option<CodeableConcept>,

    /// Relative likelihood
    #[serde(rename = "relativeRisk")]
    pub relative_risk: Option<f64>,

    /// Timeframe or age range
    pub when: Option<serde_json::Value>,

    /// Explanation of prediction
    pub rationale: Option<String>,
}

/// An assessment of the likely outcome(s) for a patient or other subject as well as the likelihood of each outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskAssessment {
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

    /// Unique identifier for the assessment
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Request fulfilled by this assessment
    #[serde(rename = "basedOn")]
    pub based_on: Option<Box<Reference>>,

    /// Part of this occurrence
    pub parent: Option<Box<Reference>>,

    /// registered | preliminary | final | amended +
    pub status: String,

    /// Evaluation mechanism
    pub method: Option<CodeableConcept>,

    /// Type of assessment
    pub code: Option<CodeableConcept>,

    /// Who/what does assessment apply to?
    pub subject: Box<Reference>,

    /// Where was assessment performed?
    pub encounter: Option<Box<Reference>>,

    /// When was assessment made?
    pub occurrence: Option<serde_json::Value>,

    /// Condition assessed
    pub condition: Option<Box<Reference>>,

    /// Who did assessment?
    pub performer: Option<Box<Reference>>,

    /// Why the assessment was necessary?
    pub reason: Option<Vec<CodeableReference>>,

    /// Information used in assessment
    pub basis: Option<Vec<Box<Reference>>>,

    /// Outcome predicted
    pub prediction: Option<Vec<RiskAssessmentPrediction>>,

    /// How to reduce risk
    pub mitigation: Option<String>,

    /// Comments on the risk assessment
    pub note: Option<Vec<Annotation>>,
}
