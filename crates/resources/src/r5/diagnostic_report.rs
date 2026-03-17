//! FHIR R5 DiagnosticReport Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Additional information supporting the diagnostic report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticReportSupportingInfo {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Supporting information role code
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Supporting information reference
    pub reference: Box<Reference>,
}

/// Key images or data associated with this report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticReportMedia {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Comment about the image or data (e.g. explanation)
    pub comment: Option<String>,

    /// Reference to the image or data source
    pub link: Box<Reference>,
}

/// The findings and interpretation of diagnostic tests performed on patients, groups of patients, products, substances, devices, and locations, and/or specimens derived from these. The report includes...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticReport {
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

    /// Business identifier for report
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// What was requested
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// registered | partial | preliminary | modified | final | amended | corrected | appended | cancelled | entered-in-error | unknown
    pub status: String,

    /// Service category
    pub category: Option<Vec<CodeableConcept>>,

    /// Name/Code for this diagnostic report
    pub code: CodeableConcept,

    /// The subject of the report - usually, but not always, the patient
    pub subject: Option<Box<Reference>>,

    /// Health care event when test ordered
    pub encounter: Option<Box<Reference>>,

    /// Clinically relevant time/time-period for report
    #[serde(rename = "effectiveDateTime")]
    pub effective_date_time: Option<String>,

    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// DateTime this version was made
    pub issued: Option<String>,

    /// Responsible Diagnostic Service
    pub performer: Option<Vec<Box<Reference>>>,

    /// Primary result interpreter
    #[serde(rename = "resultsInterpreter")]
    pub results_interpreter: Option<Vec<Box<Reference>>>,

    /// Specimens this report is based on
    pub specimen: Option<Vec<Box<Reference>>>,

    /// Observations
    pub result: Option<Vec<Box<Reference>>>,

    /// Comments about the diagnostic report
    pub note: Option<Vec<Annotation>>,

    /// Reference to full details of an analysis associated with the diagnostic report
    pub study: Option<Vec<Box<Reference>>>,

    /// Additional information supporting the diagnostic report
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<DiagnosticReportSupportingInfo>>,

    /// Key images or data associated with this report
    pub media: Option<Vec<DiagnosticReportMedia>>,

    /// Reference to a Composition resource for the DiagnosticReport structure
    pub composition: Option<Box<Reference>>,

    /// Clinical conclusion (interpretation) of test results
    pub conclusion: Option<String>,

    /// Codes for the clinical conclusion of test results
    #[serde(rename = "conclusionCode")]
    pub conclusion_code: Option<Vec<CodeableConcept>>,

    /// Entire report as issued
    #[serde(rename = "presentedForm")]
    pub presented_form: Option<Vec<Attachment>>,
}
