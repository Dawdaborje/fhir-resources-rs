//! FHIR R4B Evidence Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Evidence variable such as population, exposure, or outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceVariableDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A text description or summary of the variable
    pub description: Option<String>,

    /// Footnotes and/or explanatory notes
    pub note: Option<Vec<Annotation>>,

    /// population | subpopulation | exposure | referenceExposure | measuredVariable | confounder
    #[serde(rename = "variableRole")]
    pub variable_role: CodeableConcept,

    /// Definition of the actual variable related to the statistic(s)
    pub observed: Option<Box<Reference>>,

    /// Definition of the intended variable related to the Evidence
    pub intended: Option<Box<Reference>>,

    /// low | moderate | high | exact
    #[serde(rename = "directnessMatch")]
    pub directness_match: Option<CodeableConcept>,
}

/// Values and parameters for a single statistic
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceStatistic {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Description of content
    pub description: Option<String>,

    /// Footnotes and/or explanatory notes
    pub note: Option<Vec<Annotation>>,

    /// Type of statistic, eg relative risk
    #[serde(rename = "statisticType")]
    pub statistic_type: Option<CodeableConcept>,

    /// Associated category for categorical variable
    pub category: Option<CodeableConcept>,

    /// Statistic value
    pub quantity: Option<Quantity>,

    /// The number of events associated with the statistic
    #[serde(rename = "numberOfEvents")]
    pub number_of_events: Option<u32>,

    /// The number of participants affected
    #[serde(rename = "numberAffected")]
    pub number_affected: Option<u32>,

    /// Number of samples in the statistic
    #[serde(rename = "sampleSize")]
    pub sample_size: Option<EvidenceStatisticSampleSize>,

    /// An attribute of the Statistic
    #[serde(rename = "attributeEstimate")]
    pub attribute_estimate: Option<Vec<EvidenceStatisticAttributeEstimate>>,

    /// An aspect of the statistical model
    #[serde(rename = "modelCharacteristic")]
    pub model_characteristic: Option<Vec<EvidenceStatisticModelCharacteristic>>,
}

/// Number of samples in the statistic
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceStatisticSampleSize {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Textual description of sample size for statistic
    pub description: Option<String>,

    /// Footnote or explanatory note about the sample size
    pub note: Option<Vec<Annotation>>,

    /// Number of contributing studies
    #[serde(rename = "numberOfStudies")]
    pub number_of_studies: Option<u32>,

    /// Cumulative number of participants
    #[serde(rename = "numberOfParticipants")]
    pub number_of_participants: Option<u32>,

    /// Number of participants with known results for measured variables
    #[serde(rename = "knownDataCount")]
    pub known_data_count: Option<u32>,
}

/// An attribute of the Statistic
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceStatisticAttributeEstimate {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Textual description of the attribute estimate
    pub description: Option<String>,

    /// Footnote or explanatory note about the estimate
    pub note: Option<Vec<Annotation>>,

    /// The type of attribute estimate, eg confidence interval or p value
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The singular quantity of the attribute estimate, for attribute estimates represented as single values; also used to report unit of measure
    pub quantity: Option<Quantity>,

    /// Level of confidence interval, eg 0.95 for 95% confidence interval
    pub level: Option<f64>,

    /// Lower and upper bound values of the attribute estimate
    pub range: Option<Range>,

    /// A nested attribute estimate; which is the attribute estimate of an attribute estimate
    #[serde(rename = "attributeEstimate")]
    pub attribute_estimate: Option<Vec<EvidenceStatisticAttributeEstimate>>,
}

/// An aspect of the statistical model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceStatisticModelCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Model specification
    pub code: CodeableConcept,

    /// Numerical value to complete model specification
    pub value: Option<Quantity>,

    /// A variable adjusted for in the adjusted analysis
    pub variable: Option<Vec<EvidenceStatisticModelCharacteristicVariable>>,

    /// An attribute of the statistic used as a model characteristic
    #[serde(rename = "attributeEstimate")]
    pub attribute_estimate: Option<Vec<EvidenceStatisticAttributeEstimate>>,
}

/// A variable adjusted for in the adjusted analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceStatisticModelCharacteristicVariable {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Description of the variable
    #[serde(rename = "variableDefinition")]
    pub variable_definition: Box<Reference>,

    /// continuous | dichotomous | ordinal | polychotomous
    pub handling: Option<String>,

    /// Description for grouping of ordinal or polychotomous variables
    #[serde(rename = "valueCategory")]
    pub value_category: Option<Vec<CodeableConcept>>,

    /// Discrete value for grouping of ordinal or polychotomous variables
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Vec<Quantity>>,

    /// Range of values for grouping of ordinal or polychotomous variables
    #[serde(rename = "valueRange")]
    pub value_range: Option<Vec<Range>>,
}

/// Certainty or quality of the evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceCertainty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Textual description of certainty
    pub description: Option<String>,

    /// Footnotes and/or explanatory notes
    pub note: Option<Vec<Annotation>>,

    /// Aspect of certainty being rated
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Assessment or judgement of the aspect
    pub rating: Option<CodeableConcept>,

    /// Individual or group who did the rating
    pub rater: Option<String>,

    /// A domain or subdomain of certainty
    pub subcomponent: Option<Vec<EvidenceCertainty>>,
}

/// The Evidence Resource provides a machine-interpretable expression of an evidence concept including the evidence variables (eg population, exposures/interventions, comparators, outcomes, measured va...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Evidence {
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

    /// Canonical identifier for this evidence, represented as a globally unique URI
    pub url: Option<String>,

    /// Additional identifier for the summary
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of this summary
    pub version: Option<String>,

    /// Name for this summary (human friendly)
    pub title: Option<String>,

    /// Citation for this evidence
    #[serde(rename = "citeAs")]
    pub cite_as: Option<serde_json::Value>,

    /// draft | active | retired | unknown
    pub status: String,

    /// Date last changed
    pub date: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// When the summary was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the summary was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Who authored the content
    pub author: Option<Vec<ContactDetail>>,

    /// Who edited the content
    pub editor: Option<Vec<ContactDetail>>,

    /// Who reviewed the content
    pub reviewer: Option<Vec<ContactDetail>>,

    /// Who endorsed the content
    pub endorser: Option<Vec<ContactDetail>>,

    /// Link or citation to artifact associated with the summary
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// Description of the particular summary
    pub description: Option<String>,

    /// Declarative description of the Evidence
    pub assertion: Option<String>,

    /// Footnotes and/or explanatory notes
    pub note: Option<Vec<Annotation>>,

    /// Evidence variable such as population, exposure, or outcome
    #[serde(rename = "variableDefinition")]
    pub variable_definition: Vec<EvidenceVariableDefinition>,

    /// The method to combine studies
    #[serde(rename = "synthesisType")]
    pub synthesis_type: Option<CodeableConcept>,

    /// The type of study that produced this evidence
    #[serde(rename = "studyType")]
    pub study_type: Option<CodeableConcept>,

    /// Values and parameters for a single statistic
    pub statistic: Option<Vec<EvidenceStatistic>>,

    /// Certainty or quality of the evidence
    pub certainty: Option<Vec<EvidenceCertainty>>,
}
