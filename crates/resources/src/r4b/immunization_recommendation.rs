//! FHIR R4B ImmunizationRecommendation Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Vaccine administration recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImmunizationRecommendationRecommendation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Vaccine or vaccine group recommendation applies to
    #[serde(rename = "vaccineCode")]
    pub vaccine_code: Option<Vec<CodeableConcept>>,

    /// Disease to be immunized against
    #[serde(rename = "targetDisease")]
    pub target_disease: Option<CodeableConcept>,

    /// Vaccine which is contraindicated to fulfill the recommendation
    #[serde(rename = "contraindicatedVaccineCode")]
    pub contraindicated_vaccine_code: Option<Vec<CodeableConcept>>,

    /// Vaccine recommendation status
    #[serde(rename = "forecastStatus")]
    pub forecast_status: CodeableConcept,

    /// Vaccine administration status reason
    #[serde(rename = "forecastReason")]
    pub forecast_reason: Option<Vec<CodeableConcept>>,

    /// Dates governing proposed immunization
    #[serde(rename = "dateCriterion")]
    pub date_criterion: Option<Vec<ImmunizationRecommendationRecommendationDateCriterion>>,

    /// Protocol details
    pub description: Option<String>,

    /// Name of vaccination series
    pub series: Option<String>,

    /// Recommended dose number within series
    #[serde(rename = "doseNumber")]
    pub dose_number: Option<serde_json::Value>,

    /// Recommended number of doses for immunity
    #[serde(rename = "seriesDoses")]
    pub series_doses: Option<serde_json::Value>,

    /// Past immunizations supporting recommendation
    #[serde(rename = "supportingImmunization")]
    pub supporting_immunization: Option<Vec<Box<Reference>>>,

    /// Patient observations supporting recommendation
    #[serde(rename = "supportingPatientInformation")]
    pub supporting_patient_information: Option<Vec<Box<Reference>>>,
}

/// Dates governing proposed immunization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImmunizationRecommendationRecommendationDateCriterion {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of date
    pub code: CodeableConcept,

    /// Recommended date
    pub value: String,
}

/// A patient's point-in-time set of recommendations (i.e. forecasting) according to a published schedule with optional supporting justification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImmunizationRecommendation {
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

    /// Who this profile is for
    pub patient: Box<Reference>,

    /// Date recommendation(s) created
    pub date: String,

    /// Who is responsible for protocol
    pub authority: Option<Box<Reference>>,

    /// Vaccine administration recommendations
    pub recommendation: Vec<ImmunizationRecommendationRecommendation>,
}
