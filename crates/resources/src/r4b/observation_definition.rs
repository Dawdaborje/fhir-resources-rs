//! FHIR R4B ObservationDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Characteristics of quantitative results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationDefinitionQuantitativeDetails {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Customary unit for quantitative results
    #[serde(rename = "customaryUnit")]
    pub customary_unit: Option<CodeableConcept>,

    /// SI unit for quantitative results
    pub unit: Option<CodeableConcept>,

    /// SI to Customary unit conversion factor
    #[serde(rename = "conversionFactor")]
    pub conversion_factor: Option<f64>,

    /// Decimal precision of observation quantitative results
    #[serde(rename = "decimalPrecision")]
    pub decimal_precision: Option<i32>,
}

/// Qualified range for continuous and ordinal observation results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationDefinitionQualifiedInterval {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// reference | critical | absolute
    pub category: Option<String>,

    /// The interval itself, for continuous or ordinal observations
    pub range: Option<Range>,

    /// Range context qualifier
    pub context: Option<CodeableConcept>,

    /// Targetted population of the range
    #[serde(rename = "appliesTo")]
    pub applies_to: Option<Vec<CodeableConcept>>,

    /// male | female | other | unknown
    pub gender: Option<String>,

    /// Applicable age range, if relevant
    pub age: Option<Range>,

    /// Applicable gestational age range, if relevant
    #[serde(rename = "gestationalAge")]
    pub gestational_age: Option<Range>,

    /// Condition associated with the reference range
    pub condition: Option<String>,
}

/// Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationDefinition {
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

    /// Category of observation
    pub category: Option<Vec<CodeableConcept>>,

    /// Type of observation (code / type)
    pub code: CodeableConcept,

    /// Business identifier for this ObservationDefinition instance
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Quantity | CodeableConcept | string | boolean | integer | Range | Ratio | SampledData | time | dateTime | Period
    #[serde(rename = "permittedDataType")]
    pub permitted_data_type: Option<Vec<String>>,

    /// Multiple results allowed
    #[serde(rename = "multipleResultsAllowed")]
    pub multiple_results_allowed: Option<bool>,

    /// Method used to produce the observation
    pub method: Option<CodeableConcept>,

    /// Preferred report name
    #[serde(rename = "preferredReportName")]
    pub preferred_report_name: Option<String>,

    /// Characteristics of quantitative results
    #[serde(rename = "quantitativeDetails")]
    pub quantitative_details: Option<ObservationDefinitionQuantitativeDetails>,

    /// Qualified range for continuous and ordinal observation results
    #[serde(rename = "qualifiedInterval")]
    pub qualified_interval: Option<Vec<ObservationDefinitionQualifiedInterval>>,

    /// Value set of valid coded values for the observations conforming to this ObservationDefinition
    #[serde(rename = "validCodedValueSet")]
    pub valid_coded_value_set: Option<Box<Reference>>,

    /// Value set of normal coded values for the observations conforming to this ObservationDefinition
    #[serde(rename = "normalCodedValueSet")]
    pub normal_coded_value_set: Option<Box<Reference>>,

    /// Value set of abnormal coded values for the observations conforming to this ObservationDefinition
    #[serde(rename = "abnormalCodedValueSet")]
    pub abnormal_coded_value_set: Option<Box<Reference>>,

    /// Value set of critical coded values for the observations conforming to this ObservationDefinition
    #[serde(rename = "criticalCodedValueSet")]
    pub critical_coded_value_set: Option<Box<Reference>>,
}
