//! FHIR R4 Measure Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Population criteria group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureGroup {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Meaning of the group
    pub code: Option<CodeableConcept>,

    /// Summary description
    pub description: Option<String>,

    /// Population criteria
    pub population: Option<Vec<MeasureGroupPopulation>>,

    /// Stratifier criteria for the measure
    pub stratifier: Option<Vec<MeasureGroupStratifier>>,
}

/// Population criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureGroupPopulation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// initial-population | numerator | numerator-exclusion | denominator | denominator-exclusion | denominator-exception | measure-population | measure-population-exclusion | measure-observation
    pub code: Option<CodeableConcept>,

    /// The human readable description of this population criteria
    pub description: Option<String>,

    /// The criteria that defines this population
    pub criteria: Expression,
}

/// Stratifier criteria for the measure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureGroupStratifier {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Meaning of the stratifier
    pub code: Option<CodeableConcept>,

    /// The human readable description of this stratifier
    pub description: Option<String>,

    /// How the measure should be stratified
    pub criteria: Option<Expression>,

    /// Stratifier criteria component for the measure
    pub component: Option<Vec<MeasureGroupStratifierComponent>>,
}

/// Stratifier criteria component for the measure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureGroupStratifierComponent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Meaning of the stratifier component
    pub code: Option<CodeableConcept>,

    /// The human readable description of this stratifier component
    pub description: Option<String>,

    /// Component of how the measure should be stratified
    pub criteria: Expression,
}

/// What other data should be reported with the measure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeasureSupplementalData {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Meaning of the supplemental data
    pub code: Option<CodeableConcept>,

    /// supplemental-data | risk-adjustment-factor
    pub usage: Option<Vec<CodeableConcept>>,

    /// The human readable description of this supplemental data
    pub description: Option<String>,

    /// Expression describing additional data to be reported
    pub criteria: Expression,
}

/// The Measure resource provides the definition of a quality measure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Measure {
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

    /// Canonical identifier for this measure, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the measure
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the measure
    pub version: Option<String>,

    /// Name for this measure (computer friendly)
    pub name: Option<String>,

    /// Name for this measure (human friendly)
    pub title: Option<String>,

    /// Subordinate title of the measure
    pub subtitle: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    pub subject: Option<serde_json::Value>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the measure
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for measure (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this measure is defined
    pub purpose: Option<String>,

    /// Describes the clinical usage of the measure
    pub usage: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// When the measure was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the measure was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the measure is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// The category of the measure, such as Education, Treatment, Assessment, etc.
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

    /// Logic used by the measure
    pub library: Option<Vec<String>>,

    /// Disclaimer for use of the measure or its referenced content
    pub disclaimer: Option<String>,

    /// proportion | ratio | continuous-variable | cohort
    pub scoring: Option<CodeableConcept>,

    /// opportunity | all-or-nothing | linear | weighted
    #[serde(rename = "compositeScoring")]
    pub composite_scoring: Option<CodeableConcept>,

    /// process | outcome | structure | patient-reported-outcome | composite
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// How risk adjustment is applied for this measure
    #[serde(rename = "riskAdjustment")]
    pub risk_adjustment: Option<String>,

    /// How is rate aggregation performed for this measure
    #[serde(rename = "rateAggregation")]
    pub rate_aggregation: Option<String>,

    /// Detailed description of why the measure exists
    pub rationale: Option<String>,

    /// Summary of clinical guidelines
    #[serde(rename = "clinicalRecommendationStatement")]
    pub clinical_recommendation_statement: Option<String>,

    /// increase | decrease
    #[serde(rename = "improvementNotation")]
    pub improvement_notation: Option<CodeableConcept>,

    /// Defined terms used in the measure documentation
    pub definition: Option<Vec<String>>,

    /// Additional guidance for implementers
    pub guidance: Option<String>,

    /// Population criteria group
    pub group: Option<Vec<MeasureGroup>>,

    /// What other data should be reported with the measure
    #[serde(rename = "supplementalData")]
    pub supplemental_data: Option<Vec<MeasureSupplementalData>>,
}
