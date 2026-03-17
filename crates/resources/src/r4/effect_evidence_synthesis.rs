//! FHIR R4 EffectEvidenceSynthesis Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// What sample size was involved?
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectEvidenceSynthesisSampleSize {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Description of sample size
    pub description: Option<String>,

    /// How many studies?
    #[serde(rename = "numberOfStudies")]
    pub number_of_studies: Option<i32>,

    /// How many participants?
    #[serde(rename = "numberOfParticipants")]
    pub number_of_participants: Option<i32>,
}

/// What was the result per exposure?
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectEvidenceSynthesisResultsByExposure {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Description of results by exposure
    pub description: Option<String>,

    /// exposure | exposure-alternative
    #[serde(rename = "exposureState")]
    pub exposure_state: Option<String>,

    /// Variant exposure states
    #[serde(rename = "variantState")]
    pub variant_state: Option<CodeableConcept>,

    /// Risk evidence synthesis
    #[serde(rename = "riskEvidenceSynthesis")]
    pub risk_evidence_synthesis: Box<Reference>,
}

/// What was the estimated effect
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectEvidenceSynthesisEffectEstimate {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Description of effect estimate
    pub description: Option<String>,

    /// Type of efffect estimate
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Variant exposure states
    #[serde(rename = "variantState")]
    pub variant_state: Option<CodeableConcept>,

    /// Point estimate
    pub value: Option<f64>,

    /// What unit is the outcome described in?
    #[serde(rename = "unitOfMeasure")]
    pub unit_of_measure: Option<CodeableConcept>,

    /// How precise the estimate is
    #[serde(rename = "precisionEstimate")]
    pub precision_estimate: Option<Vec<EffectEvidenceSynthesisEffectEstimatePrecisionEstimate>>,
}

/// How precise the estimate is
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectEvidenceSynthesisEffectEstimatePrecisionEstimate {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of precision estimate
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Level of confidence interval
    pub level: Option<f64>,

    /// Lower bound
    pub from: Option<f64>,

    /// Upper bound
    pub to: Option<f64>,
}

/// How certain is the effect
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectEvidenceSynthesisCertainty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Certainty rating
    pub rating: Option<Vec<CodeableConcept>>,

    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<Annotation>>,

    /// A component that contributes to the overall certainty
    #[serde(rename = "certaintySubcomponent")]
    pub certainty_subcomponent: Option<Vec<EffectEvidenceSynthesisCertaintyCertaintySubcomponent>>,
}

/// A component that contributes to the overall certainty
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectEvidenceSynthesisCertaintyCertaintySubcomponent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of subcomponent of certainty rating
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Subcomponent certainty rating
    pub rating: Option<Vec<CodeableConcept>>,

    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<Annotation>>,
}

/// The EffectEvidenceSynthesis resource describes the difference in an outcome between exposures states in a population where the effect estimate is derived from a combination of research studies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectEvidenceSynthesis {
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

    /// Canonical identifier for this effect evidence synthesis, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the effect evidence synthesis
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the effect evidence synthesis
    pub version: Option<String>,

    /// Name for this effect evidence synthesis (computer friendly)
    pub name: Option<String>,

    /// Name for this effect evidence synthesis (human friendly)
    pub title: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the effect evidence synthesis
    pub description: Option<String>,

    /// Used for footnotes or explanatory notes
    pub note: Option<Vec<Annotation>>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for effect evidence synthesis (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// When the effect evidence synthesis was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the effect evidence synthesis was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the effect evidence synthesis is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// The category of the EffectEvidenceSynthesis, such as Education, Treatment, Assessment, etc.
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

    /// Type of synthesis
    #[serde(rename = "synthesisType")]
    pub synthesis_type: Option<CodeableConcept>,

    /// Type of study
    #[serde(rename = "studyType")]
    pub study_type: Option<CodeableConcept>,

    /// What population?
    pub population: Box<Reference>,

    /// What exposure?
    pub exposure: Box<Reference>,

    /// What comparison exposure?
    #[serde(rename = "exposureAlternative")]
    pub exposure_alternative: Box<Reference>,

    /// What outcome?
    pub outcome: Box<Reference>,

    /// What sample size was involved?
    #[serde(rename = "sampleSize")]
    pub sample_size: Option<EffectEvidenceSynthesisSampleSize>,

    /// What was the result per exposure?
    #[serde(rename = "resultsByExposure")]
    pub results_by_exposure: Option<Vec<EffectEvidenceSynthesisResultsByExposure>>,

    /// What was the estimated effect
    #[serde(rename = "effectEstimate")]
    pub effect_estimate: Option<Vec<EffectEvidenceSynthesisEffectEstimate>>,

    /// How certain is the effect
    pub certainty: Option<Vec<EffectEvidenceSynthesisCertainty>>,
}
