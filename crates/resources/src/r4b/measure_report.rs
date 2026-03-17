//! FHIR R4B MeasureReport Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
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

    /// Meaning of the group
    pub code: Option<CodeableConcept>,

    /// The populations in the group
    pub population: Option<Vec<MeasureReportGroupPopulation>>,

    /// What score this group achieved
    #[serde(rename = "measureScore")]
    pub measure_score: Option<Quantity>,

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

    /// initial-population | numerator | numerator-exclusion | denominator | denominator-exclusion | denominator-exception | measure-population | measure-population-exclusion | measure-observation
    pub code: Option<CodeableConcept>,

    /// Size of the population
    pub count: Option<i32>,

    /// For subject-list reports, the subject results in this population
    #[serde(rename = "subjectResults")]
    pub subject_results: Option<Box<Reference>>,
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

    /// What stratifier of the group
    pub code: Option<Vec<CodeableConcept>>,

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
    pub value: Option<CodeableConcept>,

    /// Stratifier component values
    pub component: Option<Vec<MeasureReportGroupStratifierStratumComponent>>,

    /// Population results in this stratum
    pub population: Option<Vec<MeasureReportGroupStratifierStratumPopulation>>,

    /// What score this stratum achieved
    #[serde(rename = "measureScore")]
    pub measure_score: Option<Quantity>,
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

    /// What stratifier component of the group
    pub code: CodeableConcept,

    /// The stratum component value, e.g. male
    pub value: CodeableConcept,
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

    /// initial-population | numerator | numerator-exclusion | denominator | denominator-exclusion | denominator-exception | measure-population | measure-population-exclusion | measure-observation
    pub code: Option<CodeableConcept>,

    /// Size of the population
    pub count: Option<i32>,

    /// For subject-list reports, the subject results in this population
    #[serde(rename = "subjectResults")]
    pub subject_results: Option<Box<Reference>>,
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

    /// individual | subject-list | summary | data-collection
    #[serde(rename = "type")]
    pub r#type: String,

    /// What measure was calculated
    pub measure: String,

    /// What individual(s) the report is for
    pub subject: Option<Box<Reference>>,

    /// When the report was generated
    pub date: Option<String>,

    /// Who is reporting the data
    pub reporter: Option<Box<Reference>>,

    /// What period the report covers
    pub period: Period,

    /// increase | decrease
    #[serde(rename = "improvementNotation")]
    pub improvement_notation: Option<CodeableConcept>,

    /// Measure results for each group
    pub group: Option<Vec<MeasureReportGroup>>,

    /// What data was used to calculate the measure score
    #[serde(rename = "evaluatedResource")]
    pub evaluated_resource: Option<Vec<Box<Reference>>>,
}
