//! FHIR R5 MeasureReport Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Measure results for each group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReportGroup {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Pointer to specific group from Measure
    #[serde(rename = "linkId")]
    pub link_id: Option<String>,

    /// Meaning of the group
    pub code: Option<CodeableConcept>,

    /// What individual(s) the report is for
    pub subject: Option<Box<Reference>>,

    /// The populations in the group
    pub population: Option<Vec<MeasureReportGroupPopulation>>,

    /// What score this group achieved
    #[serde(rename = "measureScore")]
    pub measure_score: Option<serde_json::Value>,

    /// Stratification results
    pub stratifier: Option<Vec<MeasureReportGroupStratifier>>,
}

/// The populations in the group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReportGroupPopulation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Pointer to specific population from Measure
    #[serde(rename = "linkId")]
    pub link_id: Option<String>,

    /// initial-population | numerator | numerator-exclusion | denominator | denominator-exclusion | denominator-exception | measure-population | measure-population-exclusion | measure-observation
    pub code: Option<CodeableConcept>,

    /// Size of the population
    pub count: Option<i32>,

    /// For subject-list reports, the subject results in this population
    #[serde(rename = "subjectResults")]
    pub subject_results: Option<Box<Reference>>,

    /// For subject-list reports, a subject result in this population
    #[serde(rename = "subjectReport")]
    pub subject_report: Option<Vec<Box<Reference>>>,

    /// What individual(s) in the population
    pub subjects: Option<Box<Reference>>,
}

/// Stratification results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReportGroupStratifier {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Pointer to specific stratifier from Measure
    #[serde(rename = "linkId")]
    pub link_id: Option<String>,

    /// What stratifier of the group
    pub code: Option<CodeableConcept>,

    /// Stratum results, one for each unique value, or set of values, in the stratifier, or stratifier components
    pub stratum: Option<Vec<MeasureReportGroupStratifierStratum>>,
}

/// Stratum results, one for each unique value, or set of values, in the stratifier, or stratifier components
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReportGroupStratifierStratum {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The stratum value, e.g. male
    pub value: Option<serde_json::Value>,

    /// Stratifier component values
    pub component: Option<Vec<MeasureReportGroupStratifierStratumComponent>>,

    /// Population results in this stratum
    pub population: Option<Vec<MeasureReportGroupStratifierStratumPopulation>>,

    /// What score this stratum achieved
    #[serde(rename = "measureScore")]
    pub measure_score: Option<serde_json::Value>,
}

/// Stratifier component values
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReportGroupStratifierStratumComponent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Pointer to specific stratifier component from Measure
    #[serde(rename = "linkId")]
    pub link_id: Option<String>,

    /// What stratifier component of the group
    pub code: CodeableConcept,

    /// The stratum component value, e.g. male
    pub value: serde_json::Value,
}

/// Population results in this stratum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReportGroupStratifierStratumPopulation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Pointer to specific population from Measure
    #[serde(rename = "linkId")]
    pub link_id: Option<String>,

    /// initial-population | numerator | numerator-exclusion | denominator | denominator-exclusion | denominator-exception | measure-population | measure-population-exclusion | measure-observation
    pub code: Option<CodeableConcept>,

    /// Size of the population
    pub count: Option<i32>,

    /// For subject-list reports, the subject results in this population
    #[serde(rename = "subjectResults")]
    pub subject_results: Option<Box<Reference>>,

    /// For subject-list reports, a subject result in this population
    #[serde(rename = "subjectReport")]
    pub subject_report: Option<Vec<Box<Reference>>>,

    /// What individual(s) in the population
    pub subjects: Option<Box<Reference>>,
}

/// The MeasureReport resource contains the results of the calculation of a measure; and optionally a reference to the resources involved in that calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureReport {
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

    /// Additional identifier for the MeasureReport
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// complete | pending | error
    pub status: String,

    /// individual | subject-list | summary | data-exchange
    #[serde(rename = "type")]
    pub r#type: String,

    /// incremental | snapshot
    #[serde(rename = "dataUpdateType")]
    pub data_update_type: Option<String>,

    /// What measure was calculated
    pub measure: Option<String>,

    /// What individual(s) the report is for
    pub subject: Option<Box<Reference>>,

    /// When the measure was calculated
    pub date: Option<String>,

    /// Who is reporting the data
    pub reporter: Option<Box<Reference>>,

    /// What vendor prepared the data
    #[serde(rename = "reportingVendor")]
    pub reporting_vendor: Option<Box<Reference>>,

    /// Where the reported data is from
    pub location: Option<Box<Reference>>,

    /// What period the report covers
    pub period: Period,

    /// What parameters were provided to the report
    #[serde(rename = "inputParameters")]
    pub input_parameters: Option<Box<Reference>>,

    /// What scoring method (e.g. proportion, ratio, continuous-variable)
    pub scoring: Option<CodeableConcept>,

    /// increase | decrease
    #[serde(rename = "improvementNotation")]
    pub improvement_notation: Option<CodeableConcept>,

    /// Measure results for each group
    pub group: Option<Vec<MeasureReportGroup>>,

    /// Additional information collected for the report
    #[serde(rename = "supplementalData")]
    pub supplemental_data: Option<Vec<Box<Reference>>>,

    /// What data was used to calculate the measure score
    #[serde(rename = "evaluatedResource")]
    pub evaluated_resource: Option<Vec<Box<Reference>>>,
}
