//! FHIR R4 AdverseEvent Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// The suspected agent causing the adverse event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventSuspectEntity {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Refers to the specific entity that caused the adverse event
    pub instance: Box<Reference>,

    /// Information on the possible cause of the event
    pub causality: Option<Vec<AdverseEventSuspectEntityCausality>>,
}

/// Information on the possible cause of the event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventSuspectEntityCausality {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Assessment of if the entity caused the event
    pub assessment: Option<CodeableConcept>,

    /// AdverseEvent.suspectEntity.causalityProductRelatedness
    #[serde(rename = "productRelatedness")]
    pub product_relatedness: Option<String>,

    /// AdverseEvent.suspectEntity.causalityAuthor
    pub author: Option<Box<Reference>>,

    /// ProbabilityScale | Bayesian | Checklist
    pub method: Option<CodeableConcept>,
}

/// Actual or potential/avoided event causing unintended physical injury resulting from or contributed to by medical care, a research study or other healthcare setting factors that requires additional ...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEvent {
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

    /// Business identifier for the event
    pub identifier: Option<Box<Identifier>>,

    /// actual | potential
    pub actuality: String,

    /// product-problem | product-quality | product-use-error | wrong-dose | incorrect-prescribing-information | wrong-technique | wrong-route-of-administration | wrong-rate | wrong-duration | wrong-time |...
    pub category: Option<Vec<CodeableConcept>>,

    /// Type of the event itself in relation to the subject
    pub event: Option<CodeableConcept>,

    /// Subject impacted by event
    pub subject: Box<Reference>,

    /// Encounter created as part of
    pub encounter: Option<Box<Reference>>,

    /// When the event occurred
    pub date: Option<String>,

    /// When the event was detected
    pub detected: Option<String>,

    /// When the event was recorded
    #[serde(rename = "recordedDate")]
    pub recorded_date: Option<String>,

    /// Effect on the subject due to this event
    #[serde(rename = "resultingCondition")]
    pub resulting_condition: Option<Vec<Box<Reference>>>,

    /// Location where adverse event occurred
    pub location: Option<Box<Reference>>,

    /// Seriousness of the event
    pub seriousness: Option<CodeableConcept>,

    /// mild | moderate | severe
    pub severity: Option<CodeableConcept>,

    /// resolved | recovering | ongoing | resolvedWithSequelae | fatal | unknown
    pub outcome: Option<CodeableConcept>,

    /// Who recorded the adverse event
    pub recorder: Option<Box<Reference>>,

    /// Who was involved in the adverse event or the potential adverse event
    pub contributor: Option<Vec<Box<Reference>>>,

    /// The suspected agent causing the adverse event
    #[serde(rename = "suspectEntity")]
    pub suspect_entity: Option<Vec<AdverseEventSuspectEntity>>,

    /// AdverseEvent.subjectMedicalHistory
    #[serde(rename = "subjectMedicalHistory")]
    pub subject_medical_history: Option<Vec<Box<Reference>>>,

    /// AdverseEvent.referenceDocument
    #[serde(rename = "referenceDocument")]
    pub reference_document: Option<Vec<Box<Reference>>>,

    /// AdverseEvent.study
    pub study: Option<Vec<Box<Reference>>>,
}
