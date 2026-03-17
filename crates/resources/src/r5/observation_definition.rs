//! FHIR R5 ObservationDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Set of qualified values for observation results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationDefinitionQualifiedValue {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Context qualifier for the set of qualified values
    pub context: Option<CodeableConcept>,

    /// Targetted population for the set of qualified values
    #[serde(rename = "appliesTo")]
    pub applies_to: Option<Vec<CodeableConcept>>,

    /// male | female | other | unknown
    pub gender: Option<String>,

    /// Applicable age range for the set of qualified values
    pub age: Option<Range>,

    /// Applicable gestational age range for the set of qualified values
    #[serde(rename = "gestationalAge")]
    pub gestational_age: Option<Range>,

    /// Condition associated with the set of qualified values
    pub condition: Option<String>,

    /// reference | critical | absolute
    #[serde(rename = "rangeCategory")]
    pub range_category: Option<String>,

    /// The range for continuous or ordinal observations
    pub range: Option<Range>,

    /// Value set of valid coded values as part of this set of qualified values
    #[serde(rename = "validCodedValueSet")]
    pub valid_coded_value_set: Option<String>,

    /// Value set of normal coded values as part of this set of qualified values
    #[serde(rename = "normalCodedValueSet")]
    pub normal_coded_value_set: Option<String>,

    /// Value set of abnormal coded values as part of this set of qualified values
    #[serde(rename = "abnormalCodedValueSet")]
    pub abnormal_coded_value_set: Option<String>,

    /// Value set of critical coded values as part of this set of qualified values
    #[serde(rename = "criticalCodedValueSet")]
    pub critical_coded_value_set: Option<String>,
}

/// Component results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationDefinitionComponent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of observation
    pub code: CodeableConcept,

    /// Quantity | CodeableConcept | string | boolean | integer | Range | Ratio | SampledData | time | dateTime | Period
    #[serde(rename = "permittedDataType")]
    pub permitted_data_type: Option<Vec<String>>,

    /// Unit for quantitative results
    #[serde(rename = "permittedUnit")]
    pub permitted_unit: Option<Vec<Coding>>,

    /// Set of qualified values for observation results
    #[serde(rename = "qualifiedValue")]
    pub qualified_value: Option<Vec<ObservationDefinitionQualifiedValue>>,
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

    /// Logical canonical URL to reference this ObservationDefinition (globally unique)
    pub url: Option<String>,

    /// Business identifier of the ObservationDefinition
    pub identifier: Option<Box<Identifier>>,

    /// Business version of the ObservationDefinition
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithm")]
    pub version_algorithm: Option<serde_json::Value>,

    /// Name for this ObservationDefinition (computer friendly)
    pub name: Option<String>,

    /// Name for this ObservationDefinition (human friendly)
    pub title: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// If for testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: Option<String>,

    /// The name of the individual or organization that published the ObservationDefinition
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the ObservationDefinition
    pub description: Option<String>,

    /// Content intends to support these contexts
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for this ObservationDefinition (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this ObservationDefinition is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// When ObservationDefinition was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// Date on which the asset content was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// The effective date range for the ObservationDefinition
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// Based on FHIR definition of another observation
    #[serde(rename = "derivedFromCanonical")]
    pub derived_from_canonical: Option<Vec<String>>,

    /// Based on external definition
    #[serde(rename = "derivedFromUri")]
    pub derived_from_uri: Option<Vec<String>>,

    /// Type of subject for the defined observation
    pub subject: Option<Vec<CodeableConcept>>,

    /// Desired kind of performer for such kind of observation
    #[serde(rename = "performerType")]
    pub performer_type: Option<CodeableConcept>,

    /// General type of observation
    pub category: Option<Vec<CodeableConcept>>,

    /// Type of observation
    pub code: CodeableConcept,

    /// Quantity | CodeableConcept | string | boolean | integer | Range | Ratio | SampledData | time | dateTime | Period
    #[serde(rename = "permittedDataType")]
    pub permitted_data_type: Option<Vec<String>>,

    /// Multiple results allowed for conforming observations
    #[serde(rename = "multipleResultsAllowed")]
    pub multiple_results_allowed: Option<bool>,

    /// Body part to be observed
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,

    /// Method used to produce the observation
    pub method: Option<CodeableConcept>,

    /// Kind of specimen used by this type of observation
    pub specimen: Option<Vec<Box<Reference>>>,

    /// Measurement device or model of device
    pub device: Option<Vec<Box<Reference>>>,

    /// The preferred name to be used when reporting the observation results
    #[serde(rename = "preferredReportName")]
    pub preferred_report_name: Option<String>,

    /// Unit for quantitative results
    #[serde(rename = "permittedUnit")]
    pub permitted_unit: Option<Vec<Coding>>,

    /// Set of qualified values for observation results
    #[serde(rename = "qualifiedValue")]
    pub qualified_value: Option<Vec<ObservationDefinitionQualifiedValue>>,

    /// Definitions of related resources belonging to this kind of observation group
    #[serde(rename = "hasMember")]
    pub has_member: Option<Vec<Box<Reference>>>,

    /// Component results
    pub component: Option<Vec<ObservationDefinitionComponent>>,
}
