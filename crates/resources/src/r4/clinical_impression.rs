//! FHIR R4 ClinicalImpression Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// One or more sets of investigations (signs, symptoms, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalImpressionInvestigation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A name/code for the set
    pub code: CodeableConcept,

    /// Record of a specific investigation
    pub item: Option<Vec<Box<Reference>>>,
}

/// Possible or likely findings and diagnoses
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalImpressionFinding {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// What was found
    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: Option<CodeableConcept>,

    /// What was found
    #[serde(rename = "itemReference")]
    pub item_reference: Option<Box<Reference>>,

    /// Which investigations support finding
    pub basis: Option<String>,
}

/// A record of a clinical assessment performed to determine what problem(s) may affect the patient and before planning the treatments or management strategies that are best to manage a patient's condi...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalImpression {
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

    /// in-progress | completed | entered-in-error
    pub status: String,

    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,

    /// Kind of assessment performed
    pub code: Option<CodeableConcept>,

    /// Why/how the assessment was performed
    pub description: Option<String>,

    /// Patient or group assessed
    pub subject: Box<Reference>,

    /// Encounter created as part of
    pub encounter: Option<Box<Reference>>,

    /// Time of assessment
    #[serde(rename = "effectiveDateTime")]
    pub effective_date_time: Option<String>,

    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// When the assessment was documented
    pub date: Option<String>,

    /// The clinician performing the assessment
    pub assessor: Option<Box<Reference>>,

    /// Reference to last assessment
    pub previous: Option<Box<Reference>>,

    /// Relevant impressions of patient state
    pub problem: Option<Vec<Box<Reference>>>,

    /// One or more sets of investigations (signs, symptoms, etc.)
    pub investigation: Option<Vec<ClinicalImpressionInvestigation>>,

    /// Clinical Protocol followed
    pub protocol: Option<Vec<String>>,

    /// Summary of the assessment
    pub summary: Option<String>,

    /// Possible or likely findings and diagnoses
    pub finding: Option<Vec<ClinicalImpressionFinding>>,

    /// Estimate of likely outcome
    #[serde(rename = "prognosisCodeableConcept")]
    pub prognosis_codeable_concept: Option<Vec<CodeableConcept>>,

    /// RiskAssessment expressing likely outcome
    #[serde(rename = "prognosisReference")]
    pub prognosis_reference: Option<Vec<Box<Reference>>>,

    /// Information supporting the clinical impression
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<Box<Reference>>>,

    /// Comments made about the ClinicalImpression
    pub note: Option<Vec<Annotation>>,
}
