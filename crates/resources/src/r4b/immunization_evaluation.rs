//! FHIR R4B ImmunizationEvaluation Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Describes a comparison of an immunization event against published recommendations to determine if the administration is "valid" in relation to those recommendations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImmunizationEvaluation {
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

    /// Business identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// completed | entered-in-error
    pub status: String,

    /// Who this evaluation is for
    pub patient: Box<Reference>,

    /// Date evaluation was performed
    pub date: Option<String>,

    /// Who is responsible for publishing the recommendations
    pub authority: Option<Box<Reference>>,

    /// Evaluation target disease
    #[serde(rename = "targetDisease")]
    pub target_disease: CodeableConcept,

    /// Immunization being evaluated
    #[serde(rename = "immunizationEvent")]
    pub immunization_event: Box<Reference>,

    /// Status of the dose relative to published recommendations
    #[serde(rename = "doseStatus")]
    pub dose_status: CodeableConcept,

    /// Reason for the dose status
    #[serde(rename = "doseStatusReason")]
    pub dose_status_reason: Option<Vec<CodeableConcept>>,

    /// Evaluation notes
    pub description: Option<String>,

    /// Name of vaccine series
    pub series: Option<String>,

    /// Dose number within series
    #[serde(rename = "doseNumber")]
    pub dose_number: Option<serde_json::Value>,

    /// Recommended number of doses for immunity
    #[serde(rename = "seriesDoses")]
    pub series_doses: Option<serde_json::Value>,
}
